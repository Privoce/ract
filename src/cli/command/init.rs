use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{AppComponent, ComponentState, Dashboard, State, Timeline, TimelineState},
    entry::Language,
    log::{InitLogs, Log, LogExt, LogItem, LogType},
    service,
};

pub struct InitCmd {
    state: ComponentState<InitState>,
    lang: Language,
    log: Log,
    cost: Cost,
}

impl AppComponent for InitCmd {
    type Output = ();
    type State = InitState;

    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            log: Log::default(),
            cost: Cost::default(),
        }
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        // handle service
        match self.state {
            ComponentState::Start => {
                self.log
                    .push(LogItem::info(InitLogs::Init.t(&self.lang).to_string()));
                self.cost.env_state = TimelineState::Running;
                self.state.next();
            }
            ComponentState::Run(r) => match r {
                InitState::CreateEnvFile => {
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
                InitState::CreateChain => {
                    self.handle_running(
                        || service::init::create_chain(),
                        |cost| (&mut cost.chain_progress, &mut cost.chain),
                        |cost| (&mut cost.chain_state, InitLogs::ChainSuccess),
                        |cost, e| (&mut cost.chain_state, InitLogs::ChainFailed(e)),
                    );
                    if self.cost.chain_state.is_success() {
                        self.log
                            .push(LogItem::info(InitLogs::Complete.t(&self.lang).to_string()));
                        self.state.next();
                    }
                }
            },
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(500))? {
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
    /// ## Render the init command
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let msg = Paragraph::new(self.draw_msg())
            .scroll((0, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));
        // [dashboard] -------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        dashboard.cost.replace(self.cost.env + self.cost.chain);
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
        let container_height = node1.height + node2.height;
        // [render components] ------------------------------------------------------------------------------------
        dashboard.render(
            frame,
            area,
            container_height,
            8,
            |frame, [main_area, msg_area]| {
                let [node1_area, node2_area] = Layout::vertical([
                    Constraint::Length(node1.height),
                    Constraint::Length(node2.height),
                ])
                .spacing(1)
                .areas(main_area);
                node1.render(node1_area, frame);
                node2.render(node2_area, frame);
                frame.render_widget(msg, msg_area);
            },
        );
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl InitCmd {
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
                    self.log
                        .push(LogItem::success(log.t(&self.lang).to_string()));
                }
                Err(e) => {
                    *progress = 96;
                    let (state, log) = failed(&mut self.cost, e.to_string());
                    *state = TimelineState::Failed;
                    self.log.push(LogItem::error(log.t(&self.lang).to_string()));
                }
            }
        }
    }
    fn draw_msg(&self) -> Text {
        self.log.draw_text()
    }
    fn env_progress(&self) -> u16 {
        self.cost.env_progress
    }
    fn chain_progress(&self) -> u16 {
        self.cost.chain_progress
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum InitState {
    #[default]
    CreateEnvFile,
    CreateChain,
}

impl State for InitState {
    fn next(&mut self) -> () {
        match self {
            InitState::CreateEnvFile => {
                *self = InitState::CreateChain;
            }
            InitState::CreateChain => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, InitState::CreateChain)
    }

    fn to_run_end(&mut self) -> () {
        *self = InitState::CreateChain;
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
