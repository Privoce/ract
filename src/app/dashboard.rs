use std::time::Duration;

use gen_utils::common::Os;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
    Frame,
};

use super::unicode::{CIRCLE_DOT, CIRCLE_FILLED};
use crate::{
    entry::Language,
    log::{Common, LogExt, LogType},
};

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
        // footer height: 2, padding: 0, border: 2, spacing: 4
        content_height += 8;
        content_height += offset; // offset
        content_height
    }
    pub fn render<R>(
        &self,
        frame: &mut Frame,
        area: Rect,
        main_height: u16,
        msg_height: u16,
        render_all: R,
    ) -> ()
    where
        R: FnOnce(&mut Frame, [Rect; 2]),
    {
        // [container] -----------------------------------------------------------
        let container_area = self.draw_container(frame, area);
        // [link] -----------------------------------------------------------------
        let link = self.draw_link();
        // [main layout and footer] -----------------------------------------------
        let [main_area, footer_area] = Layout::vertical([
            Constraint::Length(8 + main_height + msg_height),
            Constraint::Length(2),
        ])
        .flex(Flex::SpaceBetween)
        .areas(container_area);
        let [header_area, info_area, main_area, msg_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(main_height),
            Constraint::Length(msg_height),
        ])
        .spacing(1)
        .areas(main_area);
        // [render] ---------------------------------------------------------------
        self.render_header(header_area, frame);
        self.render_info(info_area, frame);
        frame.render_widget(link, footer_area);
        render_all(frame, [main_area, msg_area]);
    }

    pub fn draw_container(&self, frame: &mut Frame, area: Rect) -> Rect {
        let [area] = Layout::horizontal([Constraint::Min(60)]).areas(area);
        let container = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 0, 0));
        let innser_area = container.inner(area);
        frame.render_widget(container, area);
        innser_area
    }
    pub fn render_header(&self, area: Rect, frame: &mut Frame) -> () {
        let header_wrapper = Block::new().borders(Borders::BOTTOM);

        let left = Text::from(Line::from(
            Span::styled("Ract Dashboard", Color::Rgb(255, 112, 67)).bold(),
        ));

        let right = Text::from(Line::from_iter(vec![
            Span::styled(CIRCLE_FILLED, Color::Green),
            Span::styled(format!(" {}", self.ty.to_string()), Color::White),
        ]));

        let [left_area, right_area] = Layout::horizontal([
            Constraint::Length(left.width() as u16),
            Constraint::Length(right.width() as u16),
        ])
        .flex(Flex::SpaceBetween)
        .areas(header_wrapper.inner(area));
        frame.render_widget(header_wrapper, area);
        frame.render_widget(left, left_area);
        frame.render_widget(right, right_area);
    }
    pub fn render_info(&self, area: Rect, frame: &mut Frame) -> () {
        // [left] ------------------------------------------------------------------------------
        let left = Block::new();

        let left_left = Text::from(vec![
            Line::from(Common::Os.t(&self.lang)),
            Line::from(""),
            Line::from(Common::Version.t(&self.lang)),
        ]);

        let left_right = Text::from(vec![
            Line::from(Span::styled(self.os.to_string(), Color::Rgb(255, 112, 67)).bold()),
            Line::from(""),
            Line::from(Span::styled("0.2.0", Color::Rgb(255, 112, 67))),
        ]);
        // [right] -----------------------------------------------------------------------------
        let right = Block::new();

        let right_left = Text::from(vec![
            Line::from(Common::Language.t(&self.lang)),
            Line::from(""),
            Line::from(Common::Total.t(&self.lang)),
        ]);

        let right_right = Text::from(vec![
            Line::from(Span::styled(self.lang.as_str(), Color::Rgb(255, 112, 67)).bold()),
            Line::from(""),
            Line::from(Span::styled(
                self.cost
                    .map(|cost| format!("{:?}", cost))
                    .unwrap_or_else(|| "0".to_string()),
                Color::Rgb(255, 112, 67),
            )),
        ]);

        // [layout] -----------------------------------------------------------------------------
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .spacing(4)
                .areas(area);

        let [left_left_area, left_right_area] = Layout::horizontal([
            Constraint::Length(left_left.width() as u16),
            Constraint::Length(left_right.width() as u16),
        ])
        .flex(Flex::SpaceBetween)
        .areas(left.inner(left_area));

        let [right_left_area, right_right_area] = Layout::horizontal([
            Constraint::Length(right_left.width() as u16),
            Constraint::Length(right_right.width() as u16),
        ])
        .flex(Flex::SpaceBetween)
        .areas(right.inner(right_area));

        // [render] -----------------------------------------------------------------------------
        frame.render_widget(left, left_area);
        frame.render_widget(left_left, left_left_area);
        frame.render_widget(left_right, left_right_area);
        frame.render_widget(right, right_area);
        frame.render_widget(right_left, right_left_area);
        frame.render_widget(right_right, right_right_area);
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
