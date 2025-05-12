use gen_utils::common::fs;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    style::Color,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::{
    env::current_dir,
    path::PathBuf,
    process::{Child, ExitStatus},
    str::FromStr,
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};
use tui_textarea::TextArea;

use crate::{
    app::{self, AppComponent, ComponentState, Dashboard, State},
    common::Result,
    entry::{Checks, Language, Underlayer},
    log::{CheckLogs, ComponentChannel, Log, LogExt, LogItem, LogType, WasmLogs},
    service::{
        self,
        check::{check_basic, CheckItem},
        wasm::WasmArgs,
    },
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
    // channel: ComponentChannel<std::result::Result<ExitStatus, gen_utils::error::Error>>
    channel: RunChannel,
    is_running: bool,
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
            textarea: Self::init_textarea(&lang),
            channel: RunChannel {
                process: None,
                sender,
                receiver,
            },
            is_running: false,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.state {
            ComponentState::Start => {
                self.state.next();
            }
            ComponentState::Run(state) => match state {
                WasmState::Port => {}
                WasmState::Running => {
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
                        event::KeyCode::Char('q') => {
                            self.quit();
                        }
                        event::KeyCode::Enter => {
                            if self.is_run_port() {
                                // confirm the port and run
                                match u16::from_str(self.textarea.lines().join("").as_str()) {
                                    Ok(port) => {
                                        self.port = port;
                                        self.state = ComponentState::Run(WasmState::Running);
                                        self.log.push(LogItem::info(
                                            WasmLogs::Start.t(&self.lang).to_string(),
                                        ));
                                    }
                                    Err(e) => {
                                        self.textarea = Self::init_textarea(&self.lang);
                                        self.log.push(LogItem::error(
                                            WasmLogs::PortError(e.to_string())
                                                .t(&self.lang)
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
        let (msg, lines) = self.draw_msg(area.width - 4);
        let mut y = 0;
        if lines > 8 {
            y = lines - 8;
        }
        let msg = Paragraph::new(msg)
            .scroll((y, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));

        // [dashboard] ----------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Wasm;
        dashboard.cost = self.cost;
        // [render components] ------------------------------------------------------------------------------------
        let main_height = if self.is_run_port() { 4 } else { 0 };

        dashboard.render(
            frame,
            area,
            main_height,
            9,
            |frame, [main_area, msg_area]| {
                if self.is_run_port() {
                    // [ask user for port] ----------------------------------------------------------------------------------------------
                    let port_text = Line::from(WasmLogs::Port.t(&self.lang).to_string());

                    let [text_area, input_area] =
                        Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
                            .areas(main_area);

                    frame.render_widget(port_text, text_area);
                    frame.render_widget(&self.textarea, input_area);
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
    fn handle_running(&mut self) -> Result<()> {
        if self.is_running {
            return Ok(());
        }

        let path = current_dir().unwrap();
        let child = if let Some(project) = self.project.as_ref() {
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
                    msg: WasmLogs::NoRactConf.t(&self.lang).to_string(),
                })
            } else {
                service::wasm::run_wasm(path, ract_path, self.port).map_err(|e| {
                    crate::log::error::Error::Other {
                        ty: Some("Wasm".to_string()),
                        msg: e.to_string(),
                    }
                })
            }
        }?;



        self.state.next();
        Ok(())
    }

    fn draw_msg(&self, w: u16) -> (Text, u16) {
        self.log.draw_text_with_width(w)
    }
    fn is_run_port(&self) -> bool {
        matches!(self.state, ComponentState::Run(WasmState::Port))
    }
    fn init_textarea(lang: &Language) -> TextArea<'static> {
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
            textarea: Self::init_textarea(&value.1),
            channel: RunChannel {
                process: None,
                sender,
                receiver,
            },
            is_running: false,
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
    process: Option<Child>,
    sender: Sender<LogItem>,
    receiver: Receiver<LogItem>,
}
