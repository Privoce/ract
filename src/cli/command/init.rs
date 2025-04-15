use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::{Line, Text},
    DefaultTerminal, Frame,
};

use crate::{
    app::{AppComponent, Dashboard, Timeline, TimelineState},
    cli::command,
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
        match &mut self.state {
            InitState::Start => {
                self.logs
                    .push(LogItem::info(InitLogs::Init.t(&self.lang).to_string()));
                self.cost.chain_state = TimelineState::Running;
                self.state.next();
            }
            InitState::Run(run_state) => match run_state {
                RunState::CreateEnvFile(progress) => {
                    if *progress == 0 {
                        // 计算花费时间
                        let start = std::time::Instant::now();
                        let res = service::init::create_env_file();
                        let duration = start.elapsed();
                        self.cost.env = duration;
                        match res {
                            Ok(_) => {
                                *progress += 100;
                                self.cost.env_state = TimelineState::Success;
                                self.logs.push(LogItem::success(
                                    InitLogs::EnvSuccess.t(&self.lang).to_string(),
                                ));
                            }
                            Err(e) => {
                                *progress = 96;
                                self.cost.env_state = TimelineState::Failed;
                                self.logs.push(LogItem::error(
                                    InitLogs::EnvFailed(e.to_string()).t(&self.lang).to_string(),
                                ));
                            }
                        }
                    }

                    if self.cost.chain_state.is_success() {
                        self.cost.chain_state = TimelineState::Running;
                        self.state.next();
                    }
                }
                RunState::CreateChain(progress) => {}
            },
            InitState::Quit => {}
        }

        let mut do_next = false;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Enter => {
                            do_next = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if do_next {}

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
            .draw();

        let node2 = Timeline::new(InitLogs::Chain.t(&self.lang).to_string(), self.lang)
            .progress(self.chain_progress())
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

    fn render_msg(&self) -> Text {
        let items: Vec<Line> = self.logs.iter().map(|log| log.fmt_line()).collect();
        Text::from_iter(items)
    }
    fn env_progress(&self) -> u16 {
        if let InitState::Run(run_state) = self.state {
            match run_state {
                RunState::CreateEnvFile(progress) => progress,
                RunState::CreateChain(_) => 100,
            }
        } else {
            0
        }
    }
    fn chain_progress(&self) -> u16 {
        if let InitState::Run(run_state) = self.state {
            match run_state {
                RunState::CreateEnvFile(_) => 0,
                RunState::CreateChain(progress) => progress,
            }
        } else {
            0
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum InitState {
    #[default]
    Start,
    Run(RunState),
    Quit,
}

impl InitState {
    pub fn quit(&mut self) {
        *self = InitState::Quit;
    }
    pub fn is_quit(&self) -> bool {
        matches!(self, InitState::Quit)
    }
    pub fn is_start(&self) -> bool {
        matches!(self, InitState::Start)
    }
    pub fn next(&mut self) {
        match self {
            InitState::Start => {
                *self = InitState::Run(RunState::default());
            }
            InitState::Run(run_state) => match run_state {
                RunState::CreateEnvFile(_) => {
                    *self = InitState::Run(RunState::CreateChain(0));
                }
                RunState::CreateChain(_) => {
                    *self = InitState::Quit;
                }
            },
            InitState::Quit => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RunState {
    CreateEnvFile(u16),
    CreateChain(u16),
}

impl Default for RunState {
    fn default() -> Self {
        Self::CreateEnvFile(0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cost {
    pub env: Duration,
    pub env_state: TimelineState,
    pub chain: Duration,
    pub chain_state: TimelineState,
}

#[cfg(test)]
mod te {
    use crate::service;

    #[test]
    fn t() {
        let start = std::time::Instant::now();
        let res = service::init::create_env_file();
        let duration = start.elapsed();
        dbg!(duration);
    }
}
