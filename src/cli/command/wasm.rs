use gen_utils::common::stream_terminal;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use std::{
    env::current_dir,
    str::FromStr,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};
use tui_textarea::TextArea;

use crate::{
    app::{AppComponent, ComponentState, Dashboard, State},
    common::Result,
    entry::Language,
    log::{CommandType, Log, LogExt, LogItem, WasmLogs},
    service::{self, wasm::WasmArgs},
};

pub struct WasmCmd {
    state: ComponentState<WasmState>,
    lang: Language,
    log: Log,
    /// if path is None, use current dir
    project: Option<String>,
    port: u16,
    cost: Option<Duration>,
    textarea: TextArea<'static>,
    channel: RunChannel,
    is_running: bool,
    last_log_time: Option<Instant>,
    scroll_y: u16,
    control: bool,
}

impl AppComponent for WasmCmd {
    type Output = ();

    type State = WasmState;

    fn new(lang: Language) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            state: Default::default(),
            lang,
            log: Log::new(),
            project: None,
            port: 8010,
            cost: None,
            textarea: Self::init_textarea(lang),
            channel: RunChannel {
                pid: None,
                sender,
                receiver,
            },
            is_running: false,
            last_log_time: None,
            scroll_y: 0,
            control: false,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log
                    .push(LogItem::info(WasmLogs::Desc.t(self.lang).to_string()).multi());
                self.state.next();
            }
            ComponentState::Run(state) => match state {
                WasmState::Port => {}
                WasmState::Running => {
                    self.before_running();
                    self.handle_running();
                }
            },
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        };

        let timeout = self.state.timeout(|| {
            if self.is_running {
                // if over 2 s no new logs, turn to long waiting
                if let Some(last_time) = self.last_log_time {
                    if last_time.elapsed() > Duration::from_secs(2) {
                        return 1000;
                    }
                }
                20
            } else {
                20
            }
        });

        if event::poll(Duration::from_millis(timeout))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => {
                            if self.is_running {
                                self.stop_process()?;
                            } else {
                                self.quit();
                            }
                        }
                        event::KeyCode::Up => {
                            if self.is_run_port() {
                                self.textarea.input(key);
                            } else {
                                self.control = true;
                                if self.scroll_y > 0 {
                                    self.scroll_y -= 1;
                                }
                            }
                        }
                        event::KeyCode::Down => {
                            if self.is_run_port() {
                                self.textarea.input(key);
                            } else {
                                self.control = true;
                                self.scroll_y += 1;
                            }
                        }
                        event::KeyCode::Enter => {
                            if self.is_run_port() {
                                // confirm the port and run
                                match u16::from_str(self.textarea.lines().join("").as_str()) {
                                    Ok(port) => {
                                        self.port = port;
                                        self.state = ComponentState::Run(WasmState::Running);
                                        self.log.push(LogItem::info(
                                            WasmLogs::Start.t(self.lang).to_string(),
                                        ));
                                    }
                                    Err(e) => {
                                        self.textarea = Self::init_textarea(self.lang);
                                        self.log.push(LogItem::error(
                                            WasmLogs::PortError(e.to_string())
                                                .t(self.lang)
                                                .to_string(),
                                        ));
                                    }
                                }
                            }
                        }
                        _ => {
                            if self.is_run_port() {
                                // handle textarea input
                                self.textarea.input(key);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let (msg, lines) = self.log.draw_text_with_width(area.width - 4);
        if !self.control {
            if lines > 12 {
                // + 2 is for top offset
                self.scroll_y = lines - 12 + 2;
            }
        }
        let msg = Paragraph::new(msg)
            .scroll((self.scroll_y as u16, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));

        // [dashboard] ----------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = CommandType::Wasm;
        dashboard.cost = self.cost;
        // [render components] ------------------------------------------------------------------------------------
        let main_height = if self.is_run_port() { 4 } else { 1 };

        dashboard.render(
            frame,
            area,
            main_height,
            14,
            |frame, [main_area, msg_area]| {
                if self.is_run_port() {
                    // [ask user for port] ----------------------------------------------------------------------------------------------
                    let port_text = Line::from(WasmLogs::Port.t(self.lang).to_string());

                    let [text_area, input_area] =
                        Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
                            .areas(main_area);

                    frame.render_widget(port_text, text_area);
                    frame.render_widget(&self.textarea, input_area);
                } else {
                    frame
                        .render_widget(Line::from(format!("• localhost:{}", self.port)), main_area);
                }

                frame.render_widget(msg, msg_area);
            },
        );
    }

    fn state(&self) -> &ComponentState<Self::State>
    where
        Self::State: State,
    {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl WasmCmd {
    fn before_running(&mut self) -> () {
        if self.is_running {
            return;
        }
        let start = Instant::now();
        let path = current_dir().unwrap();
        let child_res = if let Some(project) = self.project.as_ref() {
            // do makepad run wasm
            service::wasm::makepad::run(path.as_path(), project, self.port).map_err(|e| {
                crate::log::error::Error::Other {
                    ty: Some("Wasm".to_string()),
                    msg: e.to_string(),
                }
            })
        } else {
            // get current dir path and check has .ract file
            let ract_path = path.join(".ract");
            if !ract_path.exists() {
                Err(crate::log::error::Error::Other {
                    ty: Some("Wasm".to_string()),
                    msg: WasmLogs::NoRactConf.t(self.lang).to_string(),
                })
            } else {
                service::wasm::run_wasm(path, ract_path, self.port).map_err(|e| {
                    crate::log::error::Error::Other {
                        ty: Some("Wasm".to_string()),
                        msg: e.to_string(),
                    }
                })
            }
        };

        match child_res {
            Ok(mut child) => {
                self.cost = Some(start.elapsed());
                self.channel.pid = Some(child.id());
                let info_sender = self.channel.sender.clone();
                let warn_sender = self.channel.sender.clone();
                let err_sender = self.channel.sender.clone();

                thread::spawn(move || {
                    let res = stream_terminal(
                        &mut child,
                        move |msg| {
                            let _ = info_sender.send(LogItem::info(msg));
                        },
                        move |msg| {
                            let _ = warn_sender.send(LogItem::warning(msg));
                        },
                    );

                    if let Err(e) = res {
                        let _ = err_sender.send(LogItem::error(e.to_string()));
                    }
                });

                self.log
                    .push(LogItem::success(WasmLogs::Package.t(self.lang).to_string()));
                self.is_running = true;
            }
            Err(e) => {
                self.log.push(LogItem::error(e.to_string()));
                self.state.to_pause();
            }
        }
    }

    fn handle_running(&mut self) -> () {
        if self.is_running {
            if let Ok(log) = self.channel.receiver.try_recv() {
                self.log.push(log);
                self.last_log_time = Some(Instant::now());
            }
        }
    }

    fn stop_process(&mut self) -> Result<()> {
        if let Some(pid) = self.channel.pid {
            // 在 macOS 上使用 kill 命令终止进程
            match std::process::Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        self.log
                            .push(LogItem::warning(WasmLogs::Stop.t(self.lang).to_string()));
                        self.state.to_pause();
                    } else {
                        self.log.push(LogItem::error(
                            WasmLogs::StopUnexpected(format!(
                                "Kill command failed with status: {}",
                                status
                            ))
                            .t(self.lang)
                            .to_string(),
                        ));
                    }
                }
                Err(e) => {
                    self.log.push(LogItem::error(
                        WasmLogs::StopUnexpected(e.to_string())
                            .t(self.lang)
                            .to_string(),
                    ));
                }
            }

            // 清除 PID
            self.channel.pid = None;
        }
        self.is_running = false;
        Ok(())
    }

    fn is_run_port(&self) -> bool {
        matches!(self.state, ComponentState::Run(WasmState::Port))
    }
    fn init_textarea(lang: Language) -> TextArea<'static> {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::bordered().border_type(BorderType::Rounded));
        textarea.set_placeholder_text(WasmLogs::Placeholder.t(lang));

        textarea
    }
}

impl From<(WasmArgs, Language)> for WasmCmd {
    fn from(value: (WasmArgs, Language)) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            state: Default::default(),
            lang: value.1,
            log: Log::new(),
            project: value.0.project,
            port: 8010,
            cost: None,
            textarea: Self::init_textarea(value.1),
            channel: RunChannel {
                pid: None,
                sender,
                receiver,
            },
            is_running: false,
            last_log_time: None,
            scroll_y: 0,
            control: false,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum WasmState {
    /// enter port
    #[default]
    Port,

    Running,
}

impl State for WasmState {
    fn next(&mut self) -> () {
        match self {
            WasmState::Port => {
                *self = WasmState::Running;
            }

            WasmState::Running => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, WasmState::Running)
    }

    fn to_run_end(&mut self) -> () {
        *self = WasmState::Running;
    }
}

struct RunChannel {
    pid: Option<u32>,
    sender: Sender<LogItem>,
    receiver: Receiver<LogItem>,
}
