use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::{Line, Text},
    DefaultTerminal, Frame,
};

use crate::{
    app::{AppComponent, Dashboard, Timeline, TimelineState},
    entry::Language,
    log::{InitLogs, LogExt, LogItem, LogType},
    service,
};

pub struct InitCmd {
    state: InitState,
    lang: Language,
    logs: Vec<LogItem>,
    cost: Cost,
}

impl AppComponent for InitCmd {
    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            logs: vec![],
            cost: Cost::default(),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        // handle service
        match self.state {
            InitState::Start => {
                self.logs
                    .push(LogItem::info(InitLogs::Init.t(&self.lang).to_string()));
                self.cost.env_state = TimelineState::Running;
                self.state.next();
            }
            InitState::Run(run_state) => match run_state {
                RunState::CreateEnvFile => {
                    self.handle_running(
                        || service::init::create_env_file(),
                        |cost| (&mut cost.env_progress, &mut cost.env),
                        |cost| (&mut cost.env_state, InitLogs::EnvSuccess),
                        |cost, e| (&mut cost.env_state, InitLogs::EnvFailed(e)),
                    );

                    if self.cost.env_state.is_success() {
                        self.cost.chain_state = TimelineState::Running;
                        self.state.next();
                    }
                }
                RunState::CreateChain => {
                    self.handle_running(
                        || service::init::create_chain(),
                        |cost| (&mut cost.chain_progress, &mut cost.chain),
                        |cost| (&mut cost.chain_state, InitLogs::ChainSuccess),
                        |cost, e| (&mut cost.chain_state, InitLogs::ChainFailed(e)),
                    );
                    if self.cost.chain_state.is_success() {
                        self.logs
                            .push(LogItem::info(InitLogs::Complete.t(&self.lang).to_string()));
                        self.state.next();
                    }
                }
            },
            InitState::Pause => {}
            InitState::Quit => {}
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

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl InitCmd {
    /// ## Render the init command
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let msg = self.render_msg();
        // [dashboard] -------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        // [render app] ------------------------------------------------------------------------------------------
        let node1 = Timeline::new(InitLogs::Env.t(&self.lang).to_string(), self.lang)
            .progress(self.env_progress())
            .cost(self.cost.env)
            .description(InitLogs::EnvDesc.t(&self.lang).to_string())
            .state(self.cost.env_state)
            .draw();

        let node2 = Timeline::new(InitLogs::Chain.t(&self.lang).to_string(), self.lang)
            .progress(self.chain_progress())
            .cost(self.cost.chain)
            .state(self.cost.chain_state)
            .draw();
        let container_height = node1.height + node2.height + 1 + 2;
        let layout = Layout::vertical([
            Constraint::Length(msg.height() as u16),
            Constraint::Length(container_height),
        ])
        .spacing(1)
        .vertical_margin(1);
        let [msg_area, dashboard_area] = layout.areas(area);
        // [render components] -------------------------------------------------------
        frame.render_widget(msg, msg_area);
        dashboard.render(frame, dashboard_area, |frame, area| {
            let [node1_area, node2_area] = Layout::vertical([
                Constraint::Length(node1.height),
                Constraint::Length(node2.height),
            ])
            .spacing(1)
            .areas(area);

            node1.render(node1_area, frame);
            node2.render(node2_area, frame);
        });
    }
    fn handle_running<S, C, Success, Failed>(
        &mut self,
        service: S,
        cost: C,
        success: Success,
        failed: Failed,
    ) where
        C: FnOnce(&mut Cost) -> (&mut u16, &mut Duration),
        S: FnOnce() -> Result<(), gen_utils::error::Error>,

        Success: FnOnce(&mut Cost) -> (&mut TimelineState, InitLogs),
        Failed: FnOnce(&mut Cost, String) -> (&mut TimelineState, InitLogs),
    {
        let (progress, cost) = cost(&mut self.cost);
        if *progress == 0 {
            let start = std::time::Instant::now();
            let res = service();
            let duration = start.elapsed();
            *cost = duration;
            match res {
                Ok(_) => {
                    *progress += 100;
                    let (state, log) = success(&mut self.cost);
                    *state = TimelineState::Success;
                    self.logs
                        .push(LogItem::success(log.t(&self.lang).to_string()));
                }
                Err(e) => {
                    *progress = 96;
                    let (state, log) = failed(&mut self.cost, e.to_string());
                    *state = TimelineState::Failed;
                    self.logs
                        .push(LogItem::error(log.t(&self.lang).to_string()));
                }
            }
        }
    }

    fn render_msg(&self) -> Text {
        let items: Vec<Line> = self.logs.iter().map(|log| log.fmt_line()).collect();
        Text::from_iter(items)
    }
    fn env_progress(&self) -> u16 {
        self.cost.env_progress
    }
    fn chain_progress(&self) -> u16 {
        self.cost.chain_progress
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum InitState {
    #[default]
    Start,
    Run(RunState),
    Pause,
    Quit,
}

impl InitState {
    pub fn quit(&mut self) {
        *self = InitState::Quit;
    }
    pub fn is_quit(&self) -> bool {
        matches!(self, InitState::Quit)
    }
    pub fn next(&mut self) {
        match self {
            InitState::Start => {
                *self = InitState::Run(RunState::default());
            }
            InitState::Run(run_state) => match run_state {
                RunState::CreateEnvFile => {
                    *self = InitState::Run(RunState::CreateChain);
                }
                RunState::CreateChain => *self = InitState::Pause,
            },
            InitState::Pause => {
                *self = InitState::Quit;
            }
            InitState::Quit => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RunState {
    CreateEnvFile,
    CreateChain,
}

impl Default for RunState {
    fn default() -> Self {
        Self::CreateEnvFile
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cost {
    pub env: Duration,
    pub env_state: TimelineState,
    pub env_progress: u16,
    pub chain: Duration,
    pub chain_state: TimelineState,
    pub chain_progress: u16,
}
