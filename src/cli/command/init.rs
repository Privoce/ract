use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Gauge, Widget},
    DefaultTerminal,
};

use crate::{app::AppTemplate, cli::command, entry::Language};

pub struct InitCmd {
    state: InitState,
    lang: Language,
    progress: f64,
}

// pub fn run(lang: &Language) -> crate::common::Result<()>{

// }

impl AppTemplate for InitCmd {
    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            progress: 0.0,
        }
    }
    fn run(mut self, terminal: &mut DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }

        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        if event::poll(Duration::from_millis(300))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => self.quit(),
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

impl Widget for &InitCmd {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([Constraint::Length(2)]);
        let [main_area] = layout.areas(area);
        self.render_gauge(main_area, buf);
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
