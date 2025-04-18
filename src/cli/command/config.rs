use std::{fs::File, time::Duration};

use crate::{
    app::{AppComponent, ComponentState, State},
    common::Result,
    entry::{ChainEnvToml, Configs, Env, Language},
    log::{error::Error, Log, LogItem},
    service,
};
use gen_utils::common::ToToml;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::{Line, Text},
    widgets::{List, ListItem},
    DefaultTerminal, Frame,
};

use super::init::InitCmd;
pub struct ConfigCmd {
    state: ComponentState<ConfigState>,
    lang: Language,
    log: Log,
    data: Option<ConfigData>,
}

impl AppComponent for ConfigCmd {
    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            log: Log::new(),
            data: None,
        }
    }

    fn run(
        mut self,
        terminal: &mut ratatui::DefaultTerminal,
        quit: bool,
    ) -> crate::common::Result<()> {
        if self.state.is_start() {
            // 加载data
            self.data.replace(ConfigData::new(self.lang, terminal)?);
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
                self.state.next();
            }
            ComponentState::Run(r) => self.handle_running(r),
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') | event::KeyCode::Enter => {
                            self.quit()
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {}

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl ConfigCmd {
    fn handle_running(&mut self, state: ConfigState) {}
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
    env: Env,
    chain_env: ChainEnvToml,
    current: Configs,
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
}
