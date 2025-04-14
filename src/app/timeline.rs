use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Padding},
    Frame,
};

use crate::entry::Language;

#[derive(Debug, Default)]
pub struct TimelineItem<'a> {
    pub name: String,
    pub description: Option<String>,
    /// time taken for the process
    pub cost: String,
    pub state: TimelineState,
    pub progress: u16,
    pub lang: Language,
    pub layout: Layout,
    pub header: (Text<'a>, Text<'a>),
    pub main: Option<Text<'a>>,
    pub footer: (Gauge<'a>, Text<'a>),
    pub height: u16,
}

impl<'a> TimelineItem<'a> {
    pub fn new(name: &str, lang: Language) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            cost: String::new(),
            state: TimelineState::default(),
            progress: 0,
            lang,
            height: 0,
            layout: Layout::default(),
            header: (Text::default(), Text::default()),
            main: None,
            footer: (Default::default(), Text::default()),
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
    /// │                                                      │
    /// │ state icon      name                                 │
    /// │                                                      │
    /// ┼──────────────────────────────────────────────────────┤
    /// │                                                      │
    /// │ description                                          │
    /// │                                                      │
    /// ┼──────────────────────────────────────────────────────┤
    /// │ ┌────────────────────────────────────────┐           │
    /// │ │                                        │ cost time │
    /// │ └────────────────────────────────────────┘           │
    /// └──────────────────────────────────────────────────────┘
    /// ```                               
    pub fn render(mut self) -> Self {
        // let header_component = Block::new().padding(Padding::new(0, 0, 0, 1));
        // let header_inner_area = header_component.inner(areas[0]);
        let (icon, color) = match self.state {
            TimelineState::UnStart => ("🟠", Color::Rgb(255, 112, 67)),
            TimelineState::Running => ("🚀", Color::Rgb(0, 255, 0)),
            TimelineState::Success => ("🟢", Color::Rgb(0, 255, 0)),
            TimelineState::Failed => ("🔴", Color::Rgb(255, 0, 0)),
        };

        self.header = (
            Text::styled(icon, color),
            Text::styled(self.name.to_string(), Color::Rgb(255, 112, 67)).bold(),
        );
        // let [state_icon_area, name_area] = Self::header_layout().areas();

        self.layout = if let Some(description) = &self.description {
            let description = Text::from(Line::from(Span::styled(description, Color::White)));
            let desc_height = description.height();
            // 2 for footer, 1 for header, 2 for spacing
            self.height = desc_height as u16 + 2 + 1 + 2;
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

        self.footer = (
            Gauge::default()
                .label("")
                .percent(self.progress)
                .gauge_style(Color::Rgb(255, 112, 67))
                .block(Block::new().bg(Color::Rgb(255, 160, 140))),
            Text::styled(format!("🎉 {}", &self.cost), Color::Gray),
        );

        self
        // let len = self.area_len();
        // let areas: Vec<Rect> = Self::layout(len).split(area).to_vec();
        // let header_component = Block::new().padding(Padding::new(0, 0, 0, 1));
        // let header_inner_area = header_component.inner(areas[0]);
        // frame.render_widget(header_component, areas[0]);

        // let [state_icon_area, name_area] = Self::header_layout().areas(header_inner_area);

        // let name_wrapper = Block::default().padding(Padding::left(2));

        // let [name_inner_area] =
        //     Layout::horizontal([Constraint::Percentage(100)]).areas(name_wrapper.inner(name_area));
        // frame.render_widget(name, name_inner_area);
        // frame.render_widget(state_icon, state_icon_area);
        // frame.render_widget(name_wrapper, name_area);

        // if let Some(description) = &self.description {
        //     let main_component = Block::new();

        //     frame.render_widget(main_component, areas[1]);
        //     let [description_area] =
        //         Layout::horizontal([Constraint::Percentage(100)]).areas(areas[1]);
        //     let description = Line::from(Span::styled(description, Color::White));
        //     frame.render_widget(description, description_area);
        // }

        // let footer_wrapper = Block::new().borders(Borders::BOTTOM);
        // let footer_area = if len == 2 { areas[1] } else { areas[2] };
        // let footer_inner_area = footer_wrapper.inner(footer_area);
        // frame.render_widget(footer_wrapper, footer_area);

        // let [progress_area, cost_area] =
        //     Self::progress_layout(cost.width()).areas(footer_inner_area);
        // frame.render_widget(progress, progress_area);
        // frame.render_widget(cost, cost_area);
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
