use std::time::Duration;

use gen_utils::common::Os;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Stylize},
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
    pub cost: Option<Duration>,
}

impl Dashboard {
    pub fn new(lang: crate::entry::Language) -> Self {
        Self {
            os: Os::current(),
            title: "Ract Dashboard".to_string(),
            lang,
            ty: LogType::Unknown,
            cost: None,
        }
    }
    pub fn height(&self, from: u16, offset: u16) -> u16 {
        let mut info_height = if self.ty.is_unknown() { 5 } else { 7 };
        if self.cost.is_some() {
            info_height += 2;
        }
        let mut content_height = if from < info_height {
            info_height
        } else {
            from
        };
        // footer height: 2, padding: 1, border: 2, spacing: 1
        content_height += 6;
        content_height += offset; // offset
        content_height
    }
    pub fn render<F>(&self, frame: &mut Frame, area: Rect, render_main: F) -> ()
    where
        F: FnOnce(&mut Frame, Rect),
    {
        // [container] -----------------------------------------------------------
        let container_area = self.draw_container(frame, area);
        // [main layout and footer] -----------------------------------------------
        let [main, footer] =
            Layout::vertical([Constraint::Length(area.height - 2), Constraint::Length(2)])
                .spacing(1)
                .areas(container_area);
        let link = self.draw_link();
        // [inner layout for left and right] -------------------------------------
        // - [left info] ---------------------------------------------------------
        let info = self.draw_info();
        let info_width = info.width();
        let [left, right] = Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Length(info_width as u16),
        ])
        .flex(Flex::SpaceBetween)
        .areas(main);

        // - [right main] --------------------------------------------------------
        render_main(frame, left);
        frame.render_widget(link, footer);
        frame.render_widget(info, right);
    }

    pub fn draw_container(&self, frame: &mut Frame, area: Rect) -> Rect {
        let [area] = Layout::horizontal([Constraint::Min(60)]).areas(area);
        let container = Block::default()
            .title(self.title.to_string())
            .title_alignment(Alignment::Left)
            .title_style(Color::Rgb(255, 112, 67))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 1, 0));
        let innser_area = container.inner(area);
        frame.render_widget(container, area);
        innser_area
    }
    pub fn draw_info(&self) -> Text {
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

        if let Some(cost) = self.cost {
            lines.push("".into());
            lines.push(Line::from_iter([
                "Total: ".into(),
                Span::styled(format!("{:?}", cost), Color::Rgb(255, 112, 67)).bold(),
            ]));
        }

        Text::from_iter(lines)
    }
    pub fn draw_link(&self) -> Text {
        Text::from_iter(vec![
            Line::from_iter(vec![
                Span::from("Github: "),
                Span::styled("https://github.com/Privoce/GenUI", Color::Rgb(255, 112, 67))
                    .bold()
                    .add_modifier(Modifier::UNDERLINED),
            ]),
            Line::from_iter(vec![
                Span::from("Doc: "),
                Span::styled(
                    "https://privoce.github.io/GenUI.github.io/tools/ract/introduction",
                    Color::Rgb(255, 112, 67),
                )
                .bold()
                .add_modifier(Modifier::UNDERLINED),
            ]),
        ])
    }
}
