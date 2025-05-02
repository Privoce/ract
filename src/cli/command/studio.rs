use crate::{
    app::{AppComponent, ComponentState, Confirm, Dashboard, State},
    common::Result,
    entry::Language,
    log::{Log, LogExt, LogItem, LogType, StudioLogs, UninstallLogs},
    service::{check::check_makepad, uninstall::uninstall_all},
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

pub struct StudioCmd {
    lang: Language,
    state: ComponentState<StudioState>,
    log: Log,
    cost: Option<Duration>,
}

impl AppComponent for StudioCmd {
    type Output = ();
    type State = StudioState;
    fn new(lang: Language) -> Self {
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            cost: None,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log.extend(vec![
                    LogItem::info(StudioLogs::Desc.t(&self.lang).to_string()).multi(),
                    LogItem::info(StudioLogs::Check.t(&self.lang).to_string()),
                ]);
                self.state.next();
            }
            ComponentState::Run(state) => {
                self.handle_running(state);
            }
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up | event::KeyCode::Down => {}
                        event::KeyCode::Enter => {
                            self.quit();
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let (msg, lines) = self.draw_msg(area.width);
        let mut y = 0;
        // here should be 8 - 1 because of the top border is 1
        if lines > 16 {
            y = lines - 16;
        }
        let msg = Paragraph::new(msg)
            .scroll((y, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));

        // [dashboard] -----------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Config;
        dashboard.cost = self.cost.clone();
        // [render] -----------------------------------------------------------
        dashboard.render(frame, area, 0, 17, |frame, [main_area, msg_area]| {
            frame.render_widget(msg, msg_area);
        });
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }
}

impl StudioCmd {
    fn draw_msg(&self, w: u16) -> (Text, u16) {
        self.log.draw_text_with_width(w - 2)
    }
    fn handle_running(&mut self, state: StudioState) {
        match state {
            StudioState::Check => {
                // [check makepad env] ------------------------------------------
                let start = Instant::now();
                let res = check_makepad();
                self.cost = Some(start.elapsed());
                match res {
                    Ok(checks) => {
                        self.log.extend(
                            checks
                                .iter()
                                .map(|item| (item, &self.lang).into())
                                .collect::<Vec<LogItem>>(),
                        );
                    }
                    Err(e) => {
                        self.log.push(LogItem::error(e.to_string()));
                    }
                }
                self.state.next();
            }
            StudioState::Running => {}
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum StudioState {
    #[default]
    Check,
    Running,
}

impl State for StudioState {
    fn next(&mut self) -> () {
        match self {
            StudioState::Check => {
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
