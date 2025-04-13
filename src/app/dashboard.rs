use std::time::Duration;

use gen_utils::common::Os;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::Color,
    text::{Line, Span},
    widgets::{block::Position, Block, BorderType, Borders, Gauge, Padding, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{entry::Language, log::LogType};

use super::AppComponent;

pub struct Dashboard {
    pub os: Os,
    pub title: String,
    pub lang: Language,
    pub ty: LogType,
}

impl Dashboard {
    pub fn new(lang: crate::entry::Language) -> Self {
        Self {
            os: Os::current(),
            title: "Ract Dashboard".to_string(),
            lang,
            ty: LogType::Unknown,
        }
    }
    pub fn render<F>(&self, frame: &mut Frame, area: Rect, render_main: F) -> ()
    where F: FnOnce(&mut Frame, Rect){
        // [container] -----------------------------------------------------------
        let container_area = self.render_container(frame, area);
        // [inner layout for left and right] -------------------------------------
        let [left, right] = Self::inner_layout().areas(container_area);
        // - [left info] ---------------------------------------------------------
        self.render_info(frame, left);
        // - [right main] --------------------------------------------------------
        render_main(frame, right);
        //
    }

    pub fn render_container(&self, frame: &mut Frame, area: Rect) -> Rect {
        let container = Block::default()
            .title(self.title.to_string())
            .title_alignment(Alignment::Left)
            .title_style(Color::Rgb(255, 112, 67))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 1, 1));
        let innser_area = container.inner(area);
        frame.render_widget(container, area);
        innser_area
    }
    pub fn inner_layout() -> Layout {
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)])
    }
    pub fn render_info(&self, frame: &mut Frame, area: Rect) -> () {
        let mut lines = vec![
            Line::from_iter([
                "Os: ".into(),
                Span::styled(self.os.to_string(), Color::Rgb(255, 112, 67)),
            ]),
            Line::from_iter([
                "Version: ".into(),
                Span::styled("0.2.0", Color::Rgb(255, 112, 67)),
            ]),
            Line::from_iter([
                "Language: ".into(),
                Span::styled(self.lang.as_str(), Color::Rgb(255, 112, 67)),
            ]),
        ];

        if !self.ty.is_unknown() {
            lines.push(Line::from_iter([
                "Type: ".into(),
                Span::styled(self.ty.to_string(), Color::Rgb(255, 112, 67)),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }
    
}
