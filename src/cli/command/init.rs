use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Gauge, List, ListItem, Paragraph, Widget},
    DefaultTerminal,
};

use crate::{
    app::{AppComponent, Dashboard},
    cli::command,
    entry::Language,
    log::LogItem,
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
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
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
            if let InitState::Start = self.state {
                self.logs.push(LogItem::info("Initializing..."));
                self.state = InitState::Run;
            } else if let InitState::Run = self.state {
                self.logs.push(LogItem::info("Running..."));
            }
        }

        Ok(())
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl Widget for &mut InitCmd {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout =
            Layout::vertical([Constraint::Max(20), Constraint::Max(20)]).vertical_margin(1);
        let [msg_area, dashboard_area] = layout.areas(area);
        // self.render_gauge(msg_area, buf);
        self.render_msg(msg_area, buf);
        self.render_dashboard(dashboard_area, buf);
    }
}

impl InitCmd {
    fn update(&mut self, width: u16) {
        if !self.state.is_start() {
            return;
        }

        self.progress += 1.0;
        if self.progress >= 100.0 {
            self.state.quit();
        }
    }
    fn render_msg(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Text::raw("Initializing...").render(area, buf);
        let items: Vec<ListItem> = self
            .logs
            .iter()
            .map(|log| ListItem::new(log.fmt_line()))
            .collect();

        List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .render(area, buf);
    }
    fn render_dashboard(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let dashboard = Dashboard::new(self.lang.clone());
        dashboard.render(area, buf);
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
    Run,
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
}
