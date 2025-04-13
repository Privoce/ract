use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Gauge, List, ListItem, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    app::{AppComponent, Dashboard},
    cli::command,
    entry::Language,
    log::{InitLogs, LogItem, LogType},
};

pub struct InitCmd {
    state: InitState,
    lang: Language,
    progress: f64,
    logs: Vec<LogItem>,
}

// pub fn run(lang: &Language) -> crate::common::Result<()>{

// }

impl AppComponent for InitCmd {
    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            progress: 0.0,
            logs: vec![],
        }
    }
    fn run(mut self, terminal: &mut DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            // terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            // self.update(terminal.size()?.width);
        }

        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
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

        if do_next {
            // handle service
            match self.state {
                InitState::Start => {
                    self.logs.push(LogItem::info(InitLogs::Init.to_string()));
                    self.state.next();
                }
                InitState::Run(run_state) => match run_state {
                    RunState::CreateEnvFile => {}
                    RunState::CreateChain => {}
                },
                InitState::Quit => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl InitCmd {
    // 渲染整个界面
    fn render_frame(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout =
            Layout::vertical([Constraint::Max(20), Constraint::Max(20)]).vertical_margin(1);
        let [msg_area, dashboard_area] = layout.areas(area);
        // [dashboard] -------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        // [render app] ------------------------------------------------------------------------------------------
        frame.render_widget(self.render_msg(), msg_area);
        dashboard.render(frame, dashboard_area, |frame, area| {
            self.render_dashboard(&dashboard, frame, area)
        });
    }

    fn render_msg(&self) -> List {
        // Text::raw("Initializing...").render(area, buf);
        let items: Vec<ListItem> = self
            .logs
            .iter()
            .map(|log| ListItem::new(log.fmt_line()))
            .collect();

        List::new(items).highlight_style(Style::default().add_modifier(Modifier::BOLD))
    }

    fn render_dashboard(
        &self,
        dashboard: &Dashboard,
        frame: &mut Frame,
        area: ratatui::prelude::Rect,
    ) {
    }

    fn render_gauge(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Gauge::default()
            .gauge_style(Color::Blue)
            .ratio(self.progress / 100.0)
            .render(area, buf);
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
                RunState::CreateEnvFile => {
                    *self = InitState::Run(RunState::CreateChain);
                }
                RunState::CreateChain => {
                    *self = InitState::Quit;
                }
            },
            InitState::Quit => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum RunState {
    #[default]
    CreateEnvFile,
    CreateChain,
}
