use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Gauge},
};

use crate::entry::Language;

#[derive(Debug, Default)]
pub struct Timeline<'a> {
    pub name: String,
    pub description: Option<String>,
    /// time taken for the process
    pub cost: String,
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
    pub fn new(name: &str, lang: Language) -> Self {
        Self {
            name: name.to_string(),
            lang,
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description.replace(description.to_string());
        self
    }

    pub fn cost(mut self, cost: &str) -> Self {
        self.cost = cost.to_string();
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
    pub fn render(mut self) -> Self {
        // [handle state] ----------------------------------------------------------------------------------------------
        let (icon, color) = match self.state {
            TimelineState::UnStart => ("🟠", Color::Rgb(255, 112, 67)),
            TimelineState::Running => ("🚀", Color::Rgb(0, 255, 0)),
            TimelineState::Success => ("🟢", Color::Rgb(0, 255, 0)),
            TimelineState::Failed => ("🔴", Color::Rgb(255, 0, 0)),
        };

        // [header] ----------------------------------------------------------------------------------------------
        self.header = self
            .header
            .state(Text::styled(icon, color))
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
        self.footer = self
            .footer
            .progress(
                Gauge::default()
                    .label("")
                    .percent(self.progress)
                    .gauge_style(Color::Rgb(255, 112, 67))
                    .block(Block::new().bg(Color::Rgb(255, 160, 140))),
            )
            .cost(Text::styled(format!("🎉 {}", &self.cost), Color::Gray))
            .draw();

        self
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
            Constraint::Percentage(76),
            Constraint::Length(self.cost.width() as u16),
        ])
        .spacing(4)
        .flex(Flex::SpaceBetween);
        self
    }
}

#[cfg(test)]
mod test {
    use super::Timeline;

    #[test]
    fn timeline() {
        let node = Timeline::new("Test", crate::entry::Language::En)
            .description("Test description")
            .render();

        dbg!(node);
    }
}
