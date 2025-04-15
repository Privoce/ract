use gen_utils::common::Os;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::{entry::Language, log::LogType};

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
    where
        F: FnOnce(&mut Frame, Rect),
    {
        // [container] -----------------------------------------------------------
        let container_area = self.render_container(frame, area);
        // [inner layout for left and right] -------------------------------------
        // - [left info] ---------------------------------------------------------
        let info = self.render_info();
        let info_width = info.width();
        let [left, right] =
            Layout::horizontal([Constraint::Length(info_width as u16), Constraint::Fill(3)])
                .spacing(4)
                .areas(container_area);

        // - [right main] --------------------------------------------------------
        render_main(frame, right);

        frame.render_widget(info, left);
    }

    pub fn render_container(&self, frame: &mut Frame, area: Rect) -> Rect {
        let container = Block::default()
            .title(self.title.to_string())
            .title_alignment(Alignment::Left)
            .title_style(Color::Rgb(255, 112, 67))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 1, 1));
        let innser_area = container.inner(area);
        frame.render_widget(container, area);
        innser_area
    }
    pub fn render_info(&self) -> Text {
        let mut lines = vec![
            Line::from_iter([
                "Os: ".into(),
                Span::styled(self.os.to_string(), Color::Rgb(255, 112, 67)).bold(),
            ]),
            "".into(),
            Line::from_iter([
                "Version: ".into(),
                Span::styled("0.2.0", Color::Rgb(255, 112, 67)).bold(),
            ]),
            "".into(),
            Line::from_iter([
                "Language: ".into(),
                Span::styled(self.lang.as_str(), Color::Rgb(255, 112, 67)).bold(),
            ]),
        ];

        if !self.ty.is_unknown() {
            lines.push("".into());
            lines.push(Line::from_iter([
                "Type: ".into(),
                Span::styled(self.ty.to_string(), Color::Rgb(255, 112, 67)).bold(),
            ]));
        }

        Text::from_iter(lines)
    }
}
