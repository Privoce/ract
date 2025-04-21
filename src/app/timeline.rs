use std::time::Duration;

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::entry::Language;

use super::unicode;

#[derive(Debug, Default)]
pub struct Timeline<'a> {
    pub name: String,
    pub description: Option<String>,
    /// time taken for the process
    pub cost: Duration,
    pub state: TimelineState,
    pub progress: u16,
    pub lang: Language,
    pub layout: Layout,
    pub header: TimelineHeader<'a>,
    pub main: Option<TimelineMain<'a>>,
    pub footer: TimelineFooter<'a>,
    pub height: u16,
}

impl<'a> Timeline<'a> {
    pub fn new(name: String, lang: Language) -> Self {
        Self {
            name,
            lang,
            ..Default::default()
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description.replace(description);
        self
    }

    pub fn cost(mut self, cost: Duration) -> Self {
        self.cost = cost;
        self
    }

    pub fn progress(mut self, progress: u16) -> Self {
        self.progress = progress;
        self
    }

    pub fn state(mut self, state: TimelineState) -> Self {
        self.state = state;
        self
    }

    pub fn layout(len: usize) -> Layout {
        return if len == 2 {
            Layout::vertical([Constraint::Length(2), Constraint::Length(2)])
        } else {
            Layout::vertical([
                Constraint::Length(2),
                Constraint::Percentage(100),
                Constraint::Length(2),
            ])
        };
    }
    pub fn header_layout() -> Layout {
        Layout::horizontal([Constraint::Length(2), Constraint::Percentage(100)]).spacing(1)
    }
    pub fn progress_layout(cost_len: usize) -> Layout {
        Layout::horizontal([
            Constraint::Percentage(76),
            Constraint::Length(cost_len as u16),
        ])
        .flex(Flex::SpaceBetween)
    }

    /// ```txt                                
    /// ┌──────────────────────────────────────────────────────┐
    /// │ state icon      name                                 │
    /// ┼──────────────────────────────────────────────────────┤
    /// │ description                                          │
    /// ┼──────────────────────────────────────────────────────┤
    /// │ ┌────────────────────────────────────────┐           │
    /// │ │                                        │ cost time │
    /// │ └────────────────────────────────────────┘           │
    /// └──────────────────────────────────────────────────────┘
    /// ```                               
    pub fn draw(mut self) -> Self {
        // [handle state] ----------------------------------------------------------------------------------------------
        let color = match self.state {
            TimelineState::UnStart => Color::Rgb(255, 112, 67),
            TimelineState::Running => Color::Blue,
            TimelineState::Success => Color::Green,
            TimelineState::Failed => Color::Red,
        };

        // [header] ----------------------------------------------------------------------------------------------
        self.header = self
            .header
            .state(Text::styled(format!("{} ", unicode::CIRCLE_DOT), color))
            .name(Text::styled(self.name.to_string(), Color::Rgb(255, 112, 67)).bold())
            .draw();

        // [main] ----------------------------------------------------------------------------------------------
        self.layout = if let Some(description) = self.description.clone() {
            let description = Text::from(Line::from(Span::styled(description, Color::White)));
            let desc_height = description.height();

            self.main
                .replace(TimelineMain::new().description(description).draw());
            // 2 for footer, 1 for header, 2 for spacing
            self.height = desc_height as u16 + 1 + 1 + 2 + 3;
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(desc_height as u16),
                Constraint::Length(2),
            ])
            .spacing(1)
        } else {
            self.height = 1 + 2 + 1;
            Layout::vertical([Constraint::Length(1), Constraint::Length(2)]).spacing(1)
        };
        // [footer] ----------------------------------------------------------------------------------------------
        let fmt_cost = if self.progress < 100 {
            format!("⏱ {:?}", &self.cost)
        } else {
            format!("🎉 {:?}", &self.cost)
        };

        self.footer = self
            .footer
            .progress(
                Gauge::default()
                    .label(format!("{}%", self.progress))
                    .percent(self.progress)
                    .gauge_style(Color::Rgb(255, 112, 67))
                    .block(Block::new().bg(Color::Rgb(255, 160, 140))),
            )
            .cost(Text::styled(fmt_cost, Color::Gray))
            .draw();

        self
    }
    pub fn render(self, area: Rect, frame: &mut Frame) {
        let header = Block::new();
        let footer = Block::new().borders(Borders::BOTTOM);

        let [header_area, footer_area] = if let Some(main) = self.main {
            let [header_area, main_area, footer_area] = self.layout.areas(area);
            frame.render_widget(main.description, main_area);
            [header_area, footer_area]
        } else {
            self.layout.areas(area)
        };

        let [header_left_area, header_right_area] =
            self.header.layout.areas(header.inner(header_area));
        let [footer_left_area, footer_right_area] =
            self.footer.layout.areas(footer.inner(footer_area));
        frame.render_widget(header, header_area);
        frame.render_widget(self.header.state, header_left_area);
        frame.render_widget(self.header.name, header_right_area);
        frame.render_widget(footer, footer_area);
        frame.render_widget(self.footer.progress, footer_left_area);
        frame.render_widget(self.footer.cost, footer_right_area);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TimelineState {
    #[default]
    UnStart,
    Running,
    Success,
    Failed,
}

impl TimelineState {
    pub fn is_success(&self) -> bool {
        matches!(self, TimelineState::Success)
    }
}

#[derive(Debug, Default)]
pub struct TimelineHeader<'h> {
    pub state: Text<'h>,
    pub name: Text<'h>,
    pub layout: Layout,
}

impl<'h> TimelineHeader<'h> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn state(mut self, state: Text<'h>) -> Self {
        self.state = state;
        self
    }
    pub fn name(mut self, name: Text<'h>) -> Self {
        self.name = name;
        self
    }
    pub fn draw(mut self) -> Self {
        self.layout = Layout::horizontal([
            Constraint::Length(self.state.width() as u16),
            Constraint::Length(self.name.width() as u16),
        ])
        .spacing(1);
        self
    }
}

#[derive(Debug, Default)]
pub struct TimelineMain<'m> {
    pub description: Text<'m>,
    pub layout: Layout,
}

impl<'m> TimelineMain<'m> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn description(mut self, description: Text<'m>) -> Self {
        self.description = description;
        self
    }
    pub fn draw(mut self) -> Self {
        self.layout = Layout::vertical([Constraint::Length(self.description.height() as u16)]);
        self
    }
}

#[derive(Debug, Default)]
pub struct TimelineFooter<'f> {
    pub progress: Gauge<'f>,
    pub cost: Text<'f>,
    pub layout: Layout,
}

impl<'f> TimelineFooter<'f> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn progress(mut self, progress: Gauge<'f>) -> Self {
        self.progress = progress;
        self
    }
    pub fn cost(mut self, cost: Text<'f>) -> Self {
        self.cost = cost;
        self
    }
    pub fn draw(mut self) -> Self {
        self.layout = Layout::horizontal([
            Constraint::Percentage(70),
            Constraint::Length(self.cost.width() as u16),
        ])
        .spacing(4)
        .flex(Flex::SpaceBetween);
        self
    }
}
