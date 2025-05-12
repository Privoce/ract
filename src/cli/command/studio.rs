use crate::{
    app::{AppComponent, ComponentState, Dashboard, Select, State, Tab},
    common::Result,
    entry::Language,
    log::{
        Common, ComponentChannel, Help, Log, LogExt, LogItem, LogType, Options, RunChannel,
        StudioLogs,
    },
    service::{
        check::check_makepad,
        studio::{default_makepad_studio_path, run_gui},
    },
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use std::{
    path::PathBuf,
    process::ExitStatus,
    thread,
    time::{Duration, Instant},
};
use tui_textarea::TextArea;

pub struct StudioCmd {
    lang: Language,
    state: ComponentState<StudioState>,
    log: Log,
    child_log: Log,
    cost: Option<Duration>,
    place: Place,
    /// log scroll y
    scroll_y: i16,
    is_default: bool,
    options: Vec<String>,
    textarea: TextArea<'static>,
    channel: ComponentChannel<std::result::Result<ExitStatus, gen_utils::error::Error>>,
    selected: bool,
    is_running: bool,
    value: String,
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
            channel: ComponentChannel::new(Some(RunChannel::new())),
            selected: false,
            scroll_y: -1,
            is_running: false,
            value: String::new(),
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
                    self.before_running();
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
                        event::KeyCode::Char('q') => match self.place.select {
                            SelectPlace::Custom => {
                                self.textarea.input(key);
                            }
                            _ => {
                                self.quit();
                            }
                        },
                        event::KeyCode::Char('l') => {
                            match self.place.current {
                                Current::Select => {
                                    self.place.current = Current::Log;
                                }
                                Current::Log => match self.place.log {
                                    LogPlace::Ract => {
                                        self.place.log = LogPlace::Studio;
                                    }
                                    LogPlace::Studio => {
                                        self.place.log = LogPlace::Ract;
                                        self.place.current = Current::Select;
                                    }
                                },
                            }
                            self.scroll_y = -1;
                        }
                        event::KeyCode::Up => match self.place.current {
                            Current::Select => match self.place.select {
                                SelectPlace::UnSelected => {
                                    self.is_default = !self.is_default;
                                }
                                SelectPlace::Default => {}
                                SelectPlace::Custom => {
                                    self.textarea.input(key);
                                }
                                _ => {}
                            },
                            Current::Log => {
                                // self.place.log.next();
                                if self.scroll_y > 0 {
                                    self.scroll_y -= 1;
                                }
                            }
                        },
                        event::KeyCode::Down => match self.place.current {
                            Current::Select => match self.place.select {
                                SelectPlace::UnSelected => {
                                    self.is_default = !self.is_default;
                                }
                                SelectPlace::Default => {}
                                SelectPlace::Custom => {
                                    self.textarea.input(key);
                                }
                                _ => {}
                            },
                            Current::Log => {
                                // self.place.log.next();
                                self.scroll_y += 1;
                            }
                        },
                        event::KeyCode::Enter => match self.place.select {
                            SelectPlace::UnSelected => {
                                if self.is_default {
                                    self.log.push(LogItem::info(
                                        StudioLogs::Gui.t(&self.lang).to_string(),
                                    ));
                                    self.place.select = SelectPlace::Default;
                                    self.state.next();
                                } else {
                                    self.place.select = SelectPlace::Custom;
                                }
                            }
                            SelectPlace::Custom => {
                                self.value = self.textarea.lines().join("");
                                self.is_default = false;
                                self.log.push(LogItem::info(
                                    StudioLogs::Custom(self.value.to_string())
                                        .t(&self.lang)
                                        .to_string(),
                                ));
                                self.state.next();
                                self.place.select = SelectPlace::Selected;
                            }
                            _ => {}
                        },
                        event::KeyCode::Esc => {
                            if self.place.select.is_custom() {
                                // self.value = self.textarea.lines().join("");
                                // self.is_default = false;
                                self.place.select = SelectPlace::UnSelected;
                            }
                        }
                        _ => {
                            if self.place.select.is_custom() {
                                self.textarea.input(key);
                            }
                        }
                    }
                } else {
                    if self.place.select.is_custom() {
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
        dashboard.ty = LogType::Studio;
        dashboard.cost = self.cost.clone();
        // [render] -----------------------------------------------------------
        let help_msg = Line::from(Common::Help(Help::Log).t(&self.lang).to_string());
        let help_msg_width = help_msg.width();
        let area_width = area.width as usize - 6;
        let help_msg_height = if help_msg_width >= area_width {
            help_msg_width / (area_width as usize) + 1
        } else {
            1
        };

        let (main_height, other_height) = match self.place.select {
            SelectPlace::UnSelected => (5 + help_msg_height, 4),
            SelectPlace::Default => (1 + help_msg_height, 0),
            SelectPlace::Custom => (4 + help_msg_height, 3),
            SelectPlace::Selected => (1 + help_msg_height, 0),
        };

        dashboard.render(
            frame,
            area,
            main_height as u16,
            14,
            |frame, [main_area, msg_area]| {
                // [select is use default or custom] --------------------------
                let help_msg = Paragraph::new(help_msg).wrap(Wrap { trim: true });

                if self.state.is_run() && !self.selected {
                    // [layout for help msg and select/input] ----------------------
                    let [help_msg_area, other_area] = Layout::vertical([
                        Constraint::Length(help_msg_height as u16),
                        Constraint::Length(other_height as u16),
                    ])
                    .flex(ratatui::layout::Flex::SpaceBetween)
                    .areas(main_area);

                    match self.place.select {
                        SelectPlace::UnSelected => {
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
                        SelectPlace::Custom => {
                            frame.render_widget(&self.textarea, other_area);
                        }
                        _ => {}
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
                let tab_index = match self.place.log {
                    LogPlace::Ract => 0,
                    LogPlace::Studio => 1,
                };
                Tab::new(vec!["Ract", "Studio"])
                    .direction(Direction::Horizontal)
                    .selected(tab_index)
                    .selected_style(
                        Style::default()
                            .fg(Color::Rgb(255, 112, 67))
                            .add_modifier(Modifier::BOLD),
                    )
                    .render(msg_tab_area, frame, |area, frame| {
                        let (msg, lines) = if tab_index == 0 {
                            // [主进程日志] ----------------------------------------
                            self.log.draw_text_with_width(area.width - 2)
                        } else {
                            // [子进程日志] ------------------------------------------------
                            self.child_log.draw_text_with_width(area.width - 2)
                        };

                        // [scroll] ------------------------------------------------
                        if self.scroll_y == -1 {
                            if lines > 12 {
                                self.scroll_y = (lines - 12) as i16;
                            } else {
                                self.scroll_y = 0;
                            }
                        }

                        let msg = Paragraph::new(msg)
                            .scroll((self.scroll_y as u16, 0))
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

    fn before_running(&mut self) {
        if self.is_running {
            return;
        }
        let start = Instant::now();
        let studio_path = if self.is_default {
            // [use default] ------------------------------------------------
            default_makepad_studio_path()
        } else {
            // [use custom] -----------------------------------------------
            let path = PathBuf::from(&self.value);
            if path.exists() {
                Ok(path)
            } else {
                Err(gen_utils::error::Error::Fs(
                    gen_utils::error::FsError::DirNotFound(path),
                ))
            }
        };
        match studio_path {
            Ok(path) => {
                self.cost.map(|cost| cost + start.elapsed());
                // get sender
                let info_sender = self.channel.sender.clone();
                let warn_sender = self.channel.sender.clone();
                let run_sender = self.channel.run_channel.as_ref().unwrap().sender.clone();
                if !self.is_running {
                    self.is_running = true;
                    thread::spawn(move || {
                        let res = run_gui(
                            path,
                            move |msg| {
                                let _ = info_sender.send(LogItem::info(msg));
                            },
                            move |msg| {
                                let _ = warn_sender.send(LogItem::warning(msg));
                            },
                        );

                        let _ = run_sender.send(res);
                    });
                }
            }
            Err(e) => {
                self.log.push(LogItem::error(
                    StudioLogs::Error(e.to_string()).t(&self.lang).to_string(),
                ));
                self.state.to_pause();
            }
        }
    }

    fn handle_running(&mut self) -> () {
        // [means the child process is running] --------------------------
        if self.is_running {
            if let Ok(res) = self
                .channel
                .run_channel
                .as_ref()
                .unwrap()
                .receiver
                .try_recv()
            {
                self.is_running = false;
                match res {
                    Ok(status) => {
                        if status.success() {
                            self.log
                                .push(LogItem::warning(StudioLogs::Stop.t(&self.lang).to_string()));
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
enum Current {
    #[default]
    Select,
    Log,
}

#[derive(Debug, Clone, Copy, Default)]
struct Place {
    select: SelectPlace,
    log: LogPlace,
    current: Current,
}

#[derive(Debug, Clone, Copy, Default)]
enum SelectPlace {
    #[default]
    UnSelected,
    Default,
    Custom,
    Selected,
}

impl SelectPlace {
    fn is_custom(&self) -> bool {
        matches!(self, SelectPlace::Custom)
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum LogPlace {
    #[default]
    Ract,
    Studio,
}
