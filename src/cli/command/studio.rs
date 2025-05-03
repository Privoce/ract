use crate::{
    app::{AppComponent, ComponentState, Confirm, Dashboard, Select, State, Tab},
    common::Result,
    entry::Language,
    log::{
        Common, ComponentChannel, Log, LogExt, LogItem, LogType, Options, StudioLogs, UninstallLogs,
    },
    service::{
        check::check_makepad,
        studio::{default_makepad_studio_path, run_gui},
        uninstall::uninstall_all,
    },
};

use gen_utils::common::fs;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tui_textarea::TextArea;

pub struct StudioCmd {
    lang: Language,
    state: ComponentState<StudioState>,
    log: Log,
    child_log: Log,
    tab_index: usize,
    cost: Option<Duration>,
    place: Place,
    /// log scroll y
    scroll_y: u16,
    is_default: bool,
    options: Vec<String>,
    textarea: TextArea<'static>,
    channel: ComponentChannel,
    selected: bool,
}

impl AppComponent for StudioCmd {
    type Output = ();
    type State = StudioState;
    fn new(lang: Language) -> Self {
        let options = vec![
            Common::Option(Options::Default).t(&lang).to_string(),
            Common::Option(Options::Custom).t(&lang).to_string(),
        ];
        let textarea = Self::init_textarea(&lang);
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            child_log: Log::new(),
            cost: None,
            place: Default::default(),
            is_default: true,
            options,
            textarea,
            channel: ComponentChannel::new(),
            selected: false,
            tab_index: 0,
            scroll_y: 0,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        self.handle_child_log();

        match self.state {
            ComponentState::Start => {
                self.log.extend(vec![
                    LogItem::info(StudioLogs::Desc.t(&self.lang).to_string()).multi(),
                    LogItem::info(StudioLogs::Check.t(&self.lang).to_string()),
                ]);
                self.state.next();
            }
            ComponentState::Run(state) => match state {
                StudioState::Check => {
                    self.handle_check();
                }
                StudioState::Select => {}
                StudioState::Running => {
                    self.handle_running();
                }
            },
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Char('l') => {
                            if self.tab_index == 0 {
                                self.tab_index = 1;
                            } else {
                                self.tab_index = 0;
                            }
                        }
                        event::KeyCode::Up | event::KeyCode::Down => {
                            if self.place.is_select() {
                                self.is_default = !self.is_default;
                            }
                        }
                        event::KeyCode::Enter => {
                            if self.place.is_select() {
                                if self.is_default {
                                    self.log.push(LogItem::info(
                                        StudioLogs::Gui.t(&self.lang).to_string(),
                                    ));
                                    self.state.next();
                                } else {
                                    self.place = Place::Input;
                                }
                                self.selected = true;
                            }
                        }
                        _ => {
                            if self.place.is_input() {
                                self.textarea.input(key);
                            }
                        }
                    }
                } else {
                    if self.place.is_input() {
                        self.textarea.input(key);
                    }
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // [dashboard] -----------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Config;
        dashboard.cost = self.cost.clone();
        // [render] -----------------------------------------------------------
        let main_height = if self.selected {
            1
        } else {
            if self.state.is_run() {
                match self.place {
                    Place::Select => 6,
                    Place::Input => 5,
                }
            } else {
                1
            }
        };

        dashboard.render(
            frame,
            area,
            main_height,
            14,
            |frame, [main_area, msg_area]| {
                // [select is use default or custom] --------------------------
                let help_msg = Line::raw("press l to focus on log tab");
                if self.state.is_run() && !self.selected {
                    // [layout for help msg and select/input] ----------------------

                    let [help_msg_area, other_area] = Layout::vertical([
                        Constraint::Length(1),
                        Constraint::Length(match self.place {
                            Place::Select => 4,
                            Place::Input => 3,
                        }),
                    ])
                    .flex(ratatui::layout::Flex::SpaceBetween)
                    .areas(main_area);

                    match self.place {
                        Place::Select => {
                            let is_default = if self.is_default { 0 } else { 1 };
                            let _ = Select::new_with_options(
                                &StudioLogs::Select.t(&self.lang).to_string(),
                                self.lang,
                                &self.options,
                                Default::default(),
                                None,
                            )
                            .selected(is_default)
                            .render_from(other_area, frame);
                        }
                        Place::Input => {
                            frame.render_widget(&self.textarea, other_area);
                        }
                    }
                    frame.render_widget(&help_msg, help_msg_area);
                } else {
                    frame.render_widget(&help_msg, main_area);
                }
                // [创建Tab来显示主进程和子进程的日志] -----------------------------
                let msg_block = Block::bordered().borders(Borders::TOP);
                let [msg_tab_area] = Layout::vertical([Constraint::Length(msg_area.height)])
                    .areas(msg_block.inner(msg_area));
                frame.render_widget(msg_block, msg_area);
                Tab::new(vec!["Ract", "Studio"])
                    .direction(Direction::Horizontal)
                    .selected(self.tab_index)
                    .selected_style(
                        Style::default()
                            .fg(Color::Rgb(255, 112, 67))
                            .add_modifier(Modifier::BOLD),
                    )
                    .render(msg_tab_area, frame, |area, frame| {
                        let (msg, lines) = if self.tab_index == 0 {
                            // [主进程日志] ----------------------------------------
                            self.log.draw_text_with_width(area.width - 2)
                        } else {
                            // [子进程日志] ------------------------------------------------
                            self.child_log.draw_text_with_width(area.width - 2)
                        };

                        if lines > 12 {
                            self.scroll_y = lines - 12;
                        }
                        let msg = Paragraph::new(msg)
                            .scroll((self.scroll_y, 0))
                            .wrap(Wrap { trim: true });

                        frame.render_widget(msg, area);
                    });
            },
        );
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }
}

impl StudioCmd {
    fn handle_child_log(&mut self) {
        if let Ok(log) = self.channel.receiver.try_recv() {
            self.child_log.push(log);
        }
    }
    fn init_textarea(lang: &Language) -> TextArea<'static> {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::bordered().border_type(BorderType::Rounded));
        textarea.set_placeholder_text(StudioLogs::Placeholder.t(lang));

        textarea
    }
    /// check makepad env
    fn handle_check(&mut self) {
        let start = Instant::now();
        let res = check_makepad();
        self.cost = Some(start.elapsed());
        match res {
            Ok(checks) => {
                let mut err = false;
                self.log.extend(
                    checks
                        .iter()
                        .map(|item| {
                            if !item.state {
                                err = true;
                            }
                            (item, &self.lang).into()
                        })
                        .collect::<Vec<LogItem>>(),
                );
                if err {
                    self.state.to_pause();
                } else {
                    self.state.next();
                }
            }
            Err(e) => {
                self.log.push(LogItem::error(e.to_string()));
                self.state.to_pause();
            }
        }
    }

    fn handle_running(&mut self) -> () {
        if self.is_default {
            // [use default] ------------------------------------------------
            let start = Instant::now();
            match default_makepad_studio_path() {
                Ok(path) => {
                    self.cost.map(|cost| cost + start.elapsed());
                    // get sender
                    let info_sender = self.channel.sender.clone();
                    let warn_sender = self.channel.sender.clone();
                    let res = run_gui(
                        path,
                        move |msg| {
                            let _ = info_sender.send(LogItem::info(msg));
                        },
                        move |msg| {
                            let _ = warn_sender.send(LogItem::warning(msg));
                        },
                    );
                    match res {
                        Ok(status) => {
                            if status.success() {
                                self.log.push(LogItem::warning(
                                    StudioLogs::Stop.t(&self.lang).to_string(),
                                ));
                            }
                            self.state.next();
                        }
                        Err(e) => {
                            self.log.push(LogItem::error(
                                StudioLogs::Error(e.to_string()).t(&self.lang).to_string(),
                            ));
                            self.state.to_pause();
                        }
                    }
                }
                Err(e) => {
                    self.log.push(LogItem::error(
                        StudioLogs::Error(e.to_string()).t(&self.lang).to_string(),
                    ));
                    self.state.to_pause();
                }
            }
        } else {
            // [use custom] -----------------------------------------------
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum StudioState {
    #[default]
    Check,
    Select,
    Running,
}

impl State for StudioState {
    fn next(&mut self) -> () {
        match self {
            StudioState::Check => {
                *self = StudioState::Select;
            }
            StudioState::Select => {
                *self = StudioState::Running;
            }
            StudioState::Running => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, StudioState::Running)
    }

    fn to_run_end(&mut self) -> () {
        *self = StudioState::Running;
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Place {
    #[default]
    Select,
    Input,
}

impl Place {
    fn next(&mut self) -> () {
        match self {
            Place::Select => {
                *self = Place::Input;
            }
            Place::Input => {}
        }
    }
    fn is_select(&self) -> bool {
        matches!(self, Place::Select)
    }
    fn is_input(&self) -> bool {
        matches!(self, Place::Input)
    }
}
