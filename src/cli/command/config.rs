use std::{
    fs::File,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::{
    app::{AppComponent, ComponentState, Dashboard, InputMode, Select, State, Tab, KV},
    common::Result,
    entry::{ChainEnvToml, Configs, Env, Language},
    log::{error::Error, Common, ConfigLogs, Help, Log, LogExt, LogItem, LogType},
    service,
};
use gen_utils::common::{fs, ToToml};
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    DefaultTerminal, Frame,
};
use tui_textarea::{Input, Key, TextArea};

use super::init::InitCmd;
pub struct ConfigCmd {
    state: ComponentState<ConfigState>,
    lang: Language,
    log: Log,
    data: Option<ConfigData>,
    cost: Option<Duration>,
    mode: InputMode,
    place: Place,
    kv_length: usize,
    kv_index: usize,
    value: String,
    textarea: TextArea<'static>,
}

impl AppComponent for ConfigCmd {
    type Outupt = ();

    fn new(lang: Language) -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        Self {
            state: Default::default(),
            lang,
            log: Log::new(),
            data: None,
            cost: None,
            mode: InputMode::default(),
            place: Place::default(),
            kv_length: 0,
            kv_index: 0,
            value: Default::default(),
            textarea,
        }
    }

    fn run(
        mut self,
        terminal: &mut ratatui::DefaultTerminal,
        quit: bool,
    ) -> crate::common::Result<Self::Outupt> {
        if self.state.is_start() {
            // 加载data
            let start = Instant::now();
            self.data.replace(ConfigData::new(self.lang, terminal)?);
            self.cost.replace(start.elapsed());
        }
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if quit && self.state.is_pause() {
                self.quit();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log.push(LogItem::success(
                    ConfigLogs::LoadSuccess.t(&self.lang).to_string(),
                ));
                if self.kv_length == 0 {
                    if let Some(data) = self.data.as_ref() {
                        self.kv_length = data.chain_env.lines_length();
                    }
                }
                self.state.next();
            }
            ComponentState::Run(r) => {
                // self.handle_running(r)
            }
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => {
                            self.quit();
                        }
                        event::KeyCode::Up => match self.place {
                            Place::Tab => {
                                if let Some(data) = self.data.as_mut() {
                                    data.next();
                                }
                            }
                            Place::Pane => {
                                if self.kv_index as isize - 1 < 0 {
                                    self.kv_index = self.kv_length - 1;
                                } else {
                                    self.kv_index -= 1;
                                }
                            }
                        },
                        event::KeyCode::Down => match self.place {
                            Place::Tab => {
                                if let Some(data) = self.data.as_mut() {
                                    data.next();
                                }
                            }
                            Place::Pane => {
                                if self.kv_index + 1 > self.kv_length - 1 {
                                    self.kv_index = 0;
                                } else {
                                    self.kv_index += 1;
                                }
                            }
                        },
                        event::KeyCode::Left => {
                            if self.place.is_tab() {
                                self.place.next();
                            } else {
                                if self.mode.is_normal() {
                                    self.place.next();
                                } else {
                                    let _ = self.textarea.input(Input {
                                        key: Key::Left,
                                        ctrl: false,
                                        alt: false,
                                        shift: false,
                                    });
                                }
                            }
                        }
                        event::KeyCode::Right => {
                            if self.place.is_tab() {
                                self.place.next();
                            } else {
                                if self.mode.is_normal() {
                                    self.place.next();
                                } else {
                                    let _ = self.textarea.input(Input {
                                        key: Key::Right,
                                        ctrl: false,
                                        alt: false,
                                        shift: false,
                                    });
                                }
                            }
                        }
                        event::KeyCode::Char('i') => {
                            if self.place.is_pane() && self.mode.is_normal() {
                                self.mode = InputMode::Edit;
                            }

                            if let InputMode::Edit = self.mode {
                                let data = self.data.as_ref().unwrap();
                                match data.current {
                                    Configs::Env => {
                                        self.value = fs::path_to_str(&data.env.0);
                                    }
                                    Configs::ChainEnvToml => {}
                                }
                            }
                        }
                        event::KeyCode::Esc => {
                            if self.place.is_pane() && self.mode.is_edit() {
                                self.mode = InputMode::Normal;
                            }
                        }
                        _ => {
                            if self.place.is_pane() && self.mode.is_edit() {
                                self.textarea.input(key);
                            }
                        }
                    }
                }else{
                    if self.place.is_pane() && self.mode.is_edit() {
                        self.textarea.input(key);
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        // self.textarea.insert_str(self.value.as_str());
        let msg = Paragraph::new(self.draw_msg())
            .scroll((0, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));
        // [dashboard] -----------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Config;
        dashboard.cost = self.cost.clone();
        // [render components] ------------------------------------------------------------------------------------
        let selected_index = self
            .data
            .as_ref()
            .map(|data| data.current_index())
            .unwrap_or(0);
        dashboard.render(frame, area, 16, 8, |frame, [main_area, msg_area]| {
            if let Some(data) = self.data.as_ref() {
                let [tab_area, input_area] =
                    Layout::vertical([Constraint::Length(13), Constraint::Length(3)])
                        .flex(Flex::SpaceBetween)
                        .areas(main_area);

                Tab::new(Configs::options())
                    .direction(Direction::Horizontal)
                    .selected(selected_index)
                    .selected_style(
                        Style::default()
                            .fg(Color::Rgb(255, 112, 67))
                            .add_modifier(Modifier::BOLD),
                    )
                    .render(tab_area, frame, |area, frame| {
                        let p = match data.current {
                            Configs::Env => {
                                vec![Line::from(fs::path_to_str(&data.env.0))]
                            }
                            Configs::ChainEnvToml => data
                                .chain_env
                                .to_lines()
                                .into_iter()
                                .enumerate()
                                .map(|(index, (k, v, is_kv))| {
                                    if is_kv {
                                        Line::from(if let Place::Pane = self.place {
                                            KV::new(k, v).selected(index == self.kv_index)
                                        } else {
                                            KV::new(k, v)
                                        })
                                    } else {
                                        Line::from(Span::styled(k, Color::Rgb(255, 112, 67)))
                                            .alignment(Alignment::Left)
                                    }
                                })
                                .collect(),
                        };

                        frame.render_widget(
                            Paragraph::new(p).wrap(Wrap { trim: true }).scroll((0, 0)),
                            area,
                        );
                    });
                // [input] ------------------------------------------------------------------------------
                // Input::new()
                //     .value(self.value.to_string())
                //     .mode(self.mode)
                //     .margin(3)
                //     .scroll_h(self.scroll_h)
                //     .render(input_area, frame);
                if self.mode.is_edit() {
                    frame.render_widget(&self.textarea, input_area);
                } else {
                    let help_msg = Common::Help(Help::Edit).t(&self.lang).to_string();
                    frame.render_widget(
                        Text::from_iter(vec![Line::raw(""), Line::raw(""), help_msg.into()]),
                        input_area,
                    );
                }
            }

            frame.render_widget(msg, msg_area);
        });

        // frame.render_widget(tab, area);
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl ConfigCmd {
    fn draw_msg(&self) -> Text {
        self.log.draw_text()
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum ConfigState {
    #[default]
    Select,
    GetSet,
}

impl State for ConfigState {
    fn next(&mut self) -> () {
        match self {
            ConfigState::Select => {
                *self = ConfigState::GetSet;
            }
            ConfigState::GetSet => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, ConfigState::GetSet)
    }

    fn to_run_end(&mut self) -> () {
        *self = ConfigState::GetSet;
    }
}

struct ConfigData {
    pub env: Env,
    pub chain_env: ChainEnvToml,
    pub current: Configs,
}

impl ConfigData {
    pub fn new(lang: Language, terminal: &mut DefaultTerminal) -> Result<Self> {
        if let Ok(env) = Env::read() {
            let chain_env =
                ChainEnvToml::try_from(env.0.to_path_buf()).map_err(|e| Error::Other {
                    ty: Some("fs".to_string()),
                    msg: e.to_string(),
                })?;

            return Ok(Self {
                env,
                chain_env,
                current: Configs::Env,
            });
        } else {
            // do init
            InitCmd::new(lang).run(terminal, true)?;
            return Self::new(lang, terminal);
        }
    }
    pub fn current_index(&self) -> usize {
        match self.current {
            Configs::Env => 0,
            Configs::ChainEnvToml => 1,
        }
    }
    pub fn next(&mut self) -> () {
        match self.current {
            Configs::Env => {
                self.current = Configs::ChainEnvToml;
            }
            Configs::ChainEnvToml => {
                self.current = Configs::Env;
            }
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum Mode {
    #[default]
    Get,
    Set,
}

impl Mode {
    pub fn next(&mut self) -> () {
        match self {
            Mode::Get => {
                *self = Mode::Set;
            }
            Mode::Set => {
                *self = Mode::Get;
            }
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum Place {
    #[default]
    Tab,
    Pane,
}

impl Place {
    pub fn next(&mut self) -> () {
        match self {
            Place::Tab => {
                *self = Place::Pane;
            }
            Place::Pane => {
                *self = Place::Tab;
            }
        }
    }
    pub fn is_pane(&self) -> bool {
        matches!(self, Place::Pane)
    }
    pub fn is_tab(&self) -> bool {
        matches!(self, Place::Tab)
    }
}
