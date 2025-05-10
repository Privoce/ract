use gen_utils::common::fs;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind}, layout::{Constraint, Layout}, style::Color, text::{Line, Text}, widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap}, Frame
};
use tui_textarea::TextArea;
use std::{path::PathBuf, str::FromStr, time::Duration};

use crate::{
    app::{self, AppComponent, ComponentState, Dashboard, State},
    common::Result,
    entry::{Checks, Language, Underlayer},
    log::{CheckLogs, Log, LogExt, LogItem, LogType, WasmLogs},
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
    path: Option<PathBuf>,
    port: u16,
    cost: Option<Duration>,
    textarea: TextArea<'static>
}

impl AppComponent for WasmCmd {
    type Output = ();

    type State = WasmState;

    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            log: Log::new(),
            path: None,
            port: 8010,
            cost: None,
            textarea: Self::init_textarea(&lang),
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.state {
            ComponentState::Start => {
                self.state.next();
            }
            ComponentState::Run(state) =>{
                match state {
                    WasmState::Port => {},
                    WasmState::Running => {},
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
                        _ => {}
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
        if lines > 7 {
            y = lines - 7;
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
         let main_height = if self.is_run_port() {
            4
         }else{
            1
         };
         
         
         dashboard.render(
             frame,
             area,
             main_height,
             8,
             |frame, [main_area, msg_area]| {
                if self.is_run_port() {
                    // [ask user for port] ----------------------------------------------------------------------------------------------
                    let port_text  = Line::from(WasmLogs::Port.t(&self.lang).to_string());

                    let [text_area, input_area] = Layout::vertical([
                        Constraint::Length(1),
                        Constraint::Length(3),
                    ]).areas(main_area);

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

impl TryFrom<(WasmArgs, Language)> for WasmCmd {
    type Error = crate::log::error::Error;

    fn try_from(value: (WasmArgs, Language)) -> std::result::Result<Self, Self::Error> {
        let path = if let Some(path) = value.0.project {
            if fs::exists_dir(&path) {
                Some(PathBuf::from(path))
            } else {
                return Err(crate::log::error::Error::Other {
                    ty: Some("Fs::DirNotFound".to_string()),
                    msg: "can not find target dir path".to_string(),
                });
            }
        } else {
            None
        };

        Ok(Self {
            state: Default::default(),
            lang: value.1,
            log: Log::new(),
            path,
            port: 8010,
            cost: None,
            textarea: Self::init_textarea(&value.1),
        })
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
