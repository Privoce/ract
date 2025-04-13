use std::time::Duration;

use gen_utils::common::Os;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::Color,
    widgets::{block::Position, Block, BorderType, Borders, Gauge, Padding, Widget},
    DefaultTerminal,
};

use crate::entry::Language;

use super::AppComponent;

pub struct Dashboard {
    pub os: Os,
    pub title: String,
    pub lang: Language,
    state: State,
}

impl AppComponent for Dashboard {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            os: Os::current(),
            title: "Ract Dashboard".to_string(),
            state: Default::default(),
            lang,
        }
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            // self.update(terminal.size()?.width);
        }

        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        if event::poll(Duration::from_millis(100))? {
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

impl Widget for &Dashboard {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([Constraint::Percentage(100)]);
        let [main_area] = layout.areas(area);
        self.render_container(main_area, buf);   
    }
}

impl Dashboard {
    pub fn render_container(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Block::default()
        .title(self.title.to_string())
        .title_alignment(Alignment::Left)
        .title_style(Color::Rgb(255, 112, 67))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(2,2,2,2))
        .render(area, buf);
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum State {
    #[default]
    Start,
    Quit,
}

impl State {
    pub fn quit(&mut self) {
        *self = State::Quit;
    }
    pub fn is_quit(&self) -> bool {
        matches!(self, State::Quit)
    }
    pub fn is_start(&self) -> bool {
        matches!(self, State::Start)
    }
}
