use std::time::{Duration, Instant};

use crate::{
    app::{unicode, AppComponent, ComponentState, Dashboard, InputMode, Select, State, Tab, KV},
    common::Result,
    entry::{ChainEnvToml, Configs, Env, Language},
    log::{error::Error, Command, Common, ConfigLogs, Fs, Help, Log, LogExt, LogItem, LogType},
};
use gen_utils::common::fs;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal,
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
    cmd_selected: usize,
}

impl AppComponent for ConfigCmd {
    type Output = ();
    type State = ConfigState;

    fn new(lang: Language) -> Self {
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
            textarea: Self::textarea_init(),
            cmd_selected: 0,
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal, quit: bool) -> Result<Self::Output>
    where
        Self: Sized,
        Self::State: State,
        Self::Output: Default,
    {
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
        fn write_success(name: String, lang: &Language) -> LogItem {
            LogItem::success(Common::Fs(Fs::WriteSuccess(name)).t(lang).to_string())
        }

        fn write_fail(reason: String, lang: &Language) -> LogItem {
            LogItem::error(Common::Fs(Fs::WriteError(reason)).t(lang).to_string())
        }

        fn write_and_store(cmp: &mut ConfigCmd, quit: bool) {
            let data = cmp.data.as_mut().unwrap();
            match data.current {
                Configs::Env => match data.env.write() {
                    Ok(_) => {
                        cmp.log.push(write_success(".env".to_string(), &cmp.lang));
                    }
                    Err(e) => {
                        cmp.log.push(write_fail(e.to_string(), &cmp.lang));
                    }
                },
                Configs::ChainEnvToml => match data.chain_env.write() {
                    Ok(_) => {
                        cmp.log
                            .push(write_success("env.toml".to_string(), &cmp.lang));
                    }
                    Err(e) => {
                        cmp.log.push(write_fail(e.to_string(), &cmp.lang));
                    }
                },
            }

            if quit {
                cmp.quit();
            }
        }

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
            _ => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => match self.place {
                            Place::Tab => {
                                self.quit();
                            }
                            Place::Pane => {
                                if self.mode.is_edit() {
                                    self.textarea.input(Input {
                                        key: Key::Char('q'),
                                        ctrl: false,
                                        alt: false,
                                        shift: false,
                                    });
                                }
                            }
                            Place::Select => {
                                self.quit();
                            }
                        },
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
                            Place::Select => {
                                if self.cmd_selected > 0 {
                                    self.cmd_selected -= 1;
                                } else {
                                    self.cmd_selected = 3;
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
                            Place::Select => {
                                if self.cmd_selected < 3 {
                                    self.cmd_selected += 1;
                                } else {
                                    self.cmd_selected = 0;
                                }
                            }
                        },
                        event::KeyCode::Left => match self.place {
                            Place::Tab => {
                                self.place = Place::Select;
                            }
                            Place::Pane => {
                                if self.mode.is_normal() {
                                    self.place = Place::Tab;
                                } else {
                                    let _ = self.textarea.input(Input {
                                        key: Key::Left,
                                        ctrl: false,
                                        alt: false,
                                        shift: false,
                                    });
                                }
                            }
                            Place::Select => {
                                self.place = Place::Tab;
                            }
                        },
                        event::KeyCode::Right => match self.place {
                            Place::Tab | Place::Select => {
                                self.place.next();
                            }
                            Place::Pane => {
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
                        },
                        event::KeyCode::Char('i') => {
                            if self.place.is_pane() && self.mode.is_normal() {
                                self.mode = InputMode::Edit;
                            }

                            if let InputMode::Edit = self.mode {
                                let data = self.data.as_ref().unwrap();
                                match data.current {
                                    Configs::Env => {
                                        self.value = fs::path_to_str(&data.env.0);
                                        // self.textarea.insert_str(self.value.as_str());
                                    }
                                    Configs::ChainEnvToml => {
                                        let (_, v_v, is_key) =
                                            &data.chain_env.to_lines()[self.kv_index];
                                        if *is_key {
                                            self.value = v_v.to_string();
                                            // self.textarea.insert_str(self.value.as_str());
                                        }
                                    }
                                }
                            }
                        }

                        event::KeyCode::Esc => {
                            match self.place {
                                Place::Tab => {
                                    self.place = Place::Select;
                                }
                                Place::Pane => {
                                    if self.mode.is_edit() {
                                        self.mode = InputMode::Normal;
                                        // save value to tmp data
                                        let data = self.data.as_mut().unwrap();
                                        match data.current {
                                            Configs::Env => {
                                                let old_value = fs::path_to_str(&data.env.0);
                                                let new_value = self.textarea.lines().join("");
                                                if old_value != new_value {
                                                    data.env.set(&new_value);
                                                    self.log.push(LogItem::warning(
                                                        Common::TmpStore(new_value)
                                                            .t(&self.lang)
                                                            .to_string(),
                                                    ));
                                                }
                                            }
                                            Configs::ChainEnvToml => {
                                                let old_value = data.chain_env.to_lines()
                                                    [self.kv_index]
                                                    .1
                                                    .to_string();
                                                let new_value = self.textarea.lines().join("");
                                                if old_value != new_value {
                                                    data.chain_env.set(self.kv_index, &new_value);
                                                    self.log.push(LogItem::warning(
                                                        Common::TmpStore(new_value)
                                                            .t(&self.lang)
                                                            .to_string(),
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        self.place.next();
                                    }
                                    // clear value
                                    self.value.clear();

                                    self.textarea = Self::textarea_init();
                                }
                                Place::Select => {}
                            }
                        }
                        event::KeyCode::Enter => {
                            match self.place {
                                Place::Tab => {}
                                Place::Pane => {
                                    if self.mode.is_edit() {
                                        self.textarea.input(Input {
                                            key: Key::Enter,
                                            ctrl: false,
                                            alt: false,
                                            shift: false,
                                        });
                                    }
                                }
                                Place::Select => {
                                    // confirm the command
                                    match Command::from_str(Command::options()[self.cmd_selected]) {
                                        Command::Q => {
                                            self.quit();
                                        }
                                        Command::Wq => {
                                            write_and_store(self, true);
                                        }
                                        Command::W => {
                                            write_and_store(self, false);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {
                            if self.place.is_pane() {
                                if self.mode.is_edit() {
                                    self.textarea.input(key);
                                }
                            }
                        }
                    }
                } else {
                    if self.place.is_pane() {
                        if self.mode.is_edit() {
                            self.textarea.input(key);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        // self.textarea.insert_str(self.value.as_str());
        // [calc scroll y] ----------------------------------------------------------------
        let (msg, lines) = self.draw_msg(area.width);
        let mut y = 0;
        // here should be 8 - 1 because of the top border is 1
        if lines > 7 {
            y = lines - 7;
        }
        let msg = Paragraph::new(msg)
            .scroll((y, 0))
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

        let input_height = match self.place {
            Place::Tab => 2,
            Place::Pane => 3,
            Place::Select => 5,
        };

        dashboard.render(
            frame,
            area,
            13 + input_height,
            8,
            |frame, [main_area, msg_area]| {
                if let Some(data) = self.data.as_ref() {
                    let [tab_area, input_area] = Layout::vertical([
                        Constraint::Length(13),
                        Constraint::Length(input_height),
                    ])
                    .spacing(1)
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
                                    let mut env_text = vec![];
                                    if self.place.is_pane() {
                                        env_text.push(Span::styled(
                                            unicode::ARROW_RIGHT_SHARP,
                                            Color::Rgb(255, 112, 67),
                                        ));
                                    }
                                    env_text.push(Span::from(fs::path_to_str(&data.env.0)));

                                    vec![Line::from(env_text)]
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

                    match self.place {
                        Place::Tab => {
                            frame.render_widget(
                                Paragraph::new(
                                    Common::Help(Help::EditComplex).t(&self.lang).to_string(),
                                )
                                .wrap(Wrap { trim: true }),
                                input_area,
                            );
                        }
                        Place::Pane => {
                            if self.mode.is_edit() {
                                frame.render_widget(&self.textarea, input_area);
                            }
                        }
                        Place::Select => {
                            Select::new_with_options(
                                Common::Command(Command::Select)
                                    .t(&self.lang)
                                    .to_string()
                                    .as_str(),
                                self.lang,
                                &Command::options(),
                                Style::default(),
                                None,
                            )
                            .selected(self.cmd_selected)
                            .render_from(input_area, frame);
                        }
                    }
                }

                frame.render_widget(msg, msg_area);
            },
        );

        // frame.render_widget(tab, area);
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl ConfigCmd {
    fn draw_msg(&self, w: u16) -> (Text, u16) {
        self.log.draw_text_with_width(w)
    }
    fn textarea_init() -> TextArea<'static> {
        let mut textarea = TextArea::default();
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        textarea
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum ConfigState {
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
enum Place {
    #[default]
    Tab,
    Pane,
    Select,
}

#[allow(unused)]
impl Place {
    pub fn next(&mut self) -> () {
        match self {
            Place::Tab => {
                *self = Place::Pane;
            }
            Place::Pane => {
                *self = Place::Select;
            }
            Place::Select => {
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
    pub fn is_select(&self) -> bool {
        matches!(self, Place::Select)
    }
}
