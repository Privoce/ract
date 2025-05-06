use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    Frame,
};
use std::time::Duration;

use crate::{
    entry::Language,
    log::{error::Error, Common, Help, LogExt},
};

use super::{unicode, AppComponent, BaseRunState, ComponentState};

/// # Select
/// single select component for terminal
/// ## UI
/// ```
/// ? ${title} [${placeholder}]
///   » ${options}
///        ...
/// [${help_msg}]
/// ```
#[derive(Debug, Clone)]
pub struct Select<'s> {
    pub options: Vec<Span<'s>>,
    pub selected: usize,
    pub placeholder: Option<Text<'s>>,
    pub title: Text<'s>,
    pub help_msg: Text<'s>,
    pub select_style: Style,
    pub option_style: Style,
    #[allow(unused)]
    pub lang: Language,
    pub state: ComponentState<BaseRunState>,
}

impl<'s> Default for Select<'s> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            selected: Default::default(),
            placeholder: Default::default(),
            title: Default::default(),
            help_msg: Text::from(Line::styled(
                format!("[ {} ]", Common::Help(Help::Select).t(&Language::En)),
                Color::Blue,
            )),
            select_style: Style::default().fg(Color::Rgb(255, 112, 67)),
            option_style: Default::default(),
            lang: Default::default(),
            state: Default::default(),
        }
    }
}

#[allow(unused)]
impl<'s> Select<'s> {
    pub fn new_with_options<S>(
        title: &'s str,
        lang: Language,
        options: &Vec<S>,
        option_style: Style,
        icon: Option<&'s str>,
    ) -> Self
    where
        S: ToString,
    {
        let options = options
            .iter()
            .map(|s| Span::styled(s.to_string(), option_style))
            .collect::<Vec<_>>();
        let icon = Span::styled(icon.unwrap_or("?"), Color::Rgb(255, 112, 67));
        let title = Text::from(Line::from_iter(vec![
            icon,
            Span::from(" "),
            Span::from(title).bold(),
        ]));
        let help_msg = Text::from(Line::styled(
            format!("[ {} ]", Common::Help(Help::Select).t(&lang)),
            Color::Blue,
        ));
        let select_style = Style::default().fg(Color::Rgb(255, 112, 67));
        Self {
            options,
            selected: 0,
            placeholder: None,
            title,
            help_msg,
            select_style,
            option_style,
            lang,
            state: Default::default(),
        }
    }
    pub fn placeholder(mut self, placeholder: Text<'s>) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
    pub fn help_msg(mut self, help_msg: Text<'s>) -> Self {
        self.help_msg = help_msg;
        self
    }
    pub fn select_style(mut self, select_style: Style) -> Self {
        self.select_style = select_style;
        self
    }
    pub fn render_from(&mut self, area: Rect, frame: &mut Frame) -> () {
        let [title_area, select_area, help_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.options.len() as u16),
            Constraint::Length(1),
        ])
        .areas(area);
        // [title] --------------------------------------------------------------------
        let placeholder_width = if let Some(placeholder) = self.placeholder.as_ref() {
            placeholder.width() as u16
        } else {
            0
        };

        let [title_left_area, title_right_area] = Layout::horizontal([
            Constraint::Length(self.title.width() as u16),
            Constraint::Length(placeholder_width),
        ])
        .spacing(2)
        .areas(title_area);
        // [select] -------------------------------------------------------------------
        let option_layout = Layout::vertical(vec![Constraint::Length(1); self.options.len()]);
        let option_areas = option_layout.split(select_area);
        // [render] ----------------------------------------------------------------------
        frame.render_widget(&self.title, title_left_area);
        if let Some(placeholder) = self.placeholder.as_ref() {
            frame.render_widget(placeholder, title_right_area);
        }
        for (i, option) in self.options.iter().enumerate() {
            let [choose_area, option_line_area] =
                Layout::horizontal([Constraint::Length(4), Constraint::Fill(1)])
                    .areas(option_areas[i]);

            let is_selected = i == self.selected;
            let option = option.clone();
            if is_selected {
                frame.render_widget(
                    Span::styled(
                        format!("  {} ", unicode::ARROW_DOUBLE_RIGHT),
                        Color::Rgb(255, 112, 67),
                    ),
                    choose_area,
                );
                frame.render_widget(option.style(self.select_style), option_line_area);
            } else {
                frame.render_widget(option.style(self.option_style), option_line_area);
            }
        }
        // [help] ----------------------------------------------------------------------
        frame.render_widget(self.help_msg.clone(), help_area);
    }
    pub fn height(&self, w: u16) -> u16 {
        fn handle(w1: u16, w2: u16) -> u16 {
            if w1 > w2 {
                w1 / w2 + 1
            } else {
                1
            }
        }

        let mut height = 0;
        height += self.options.len() as u16;

        let placeholder_width = if let Some(placeholder) = self.placeholder.as_ref() {
            placeholder.width() as u16
        } else {
            0
        };

        height += handle(self.title.width() as u16 + placeholder_width, w);
        height += handle(self.help_msg.width() as u16, w);

        height
    }
}

impl<'s> AppComponent for Select<'s> {
    type Output = usize;
    type State = BaseRunState;

    fn new(lang: Language) -> Self {
        Self {
            lang,
            ..Default::default()
        }
    }

    fn run(
        mut self,
        terminal: &mut ratatui::DefaultTerminal,
        quit: bool,
    ) -> crate::common::Result<Self::Output> {
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if quit && self.state.is_pause() {
                self.quit();
            }
        }
        Ok(self.selected)
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => {
                            // err
                            return Err(Error::AppExit);
                        }
                        event::KeyCode::Down => {
                            if self.selected < self.options.len() - 1 {
                                self.selected += 1;
                            } else {
                                self.selected = 0;
                            }
                        }
                        event::KeyCode::Up => {
                            if self.selected > 0 {
                                self.selected -= 1;
                            } else {
                                self.selected = self.options.len() - 1;
                            }
                        }
                        event::KeyCode::Enter => {
                            self.quit();
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.render_from(area, frame);
    }

    fn state(&self) -> &ComponentState<Self::State>
    where
        Self::State: super::State,
    {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

pub struct Confirm<'c>(pub Select<'c>);

impl<'c> Confirm<'c> {
    pub fn new(title: &'c str, lang: Language) -> Self {
        Self(Select::new_with_options(
            title,
            lang,
            &vec!["Yes", "No"],
            Default::default(),
            Some(unicode::WARNING),
        ))
    }
}

/// # Multi Select
/// single select component for terminal
/// ## UI
/// ```
/// ? ${title} [${placeholder}]
///   » [ ] ${options}
///        ...
/// [${help_msg}]
/// ```
#[derive(Debug, Clone)]
pub struct MultiSelect<'s> {
    pub options: Vec<Span<'s>>,
    pub selected: Vec<usize>,
    pub placeholder: Option<Text<'s>>,
    pub title: Text<'s>,
    pub help_msg: Text<'s>,
    pub select_style: Style,
    pub option_style: Style,
    #[allow(unused)]
    pub lang: Language,
    pub state: ComponentState<BaseRunState>,
}

impl<'s> Default for MultiSelect<'s> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            selected: Default::default(),
            placeholder: Default::default(),
            title: Default::default(),
            help_msg: Text::from(Line::styled(
                format!("[ {} ]", Common::Help(Help::Select).t(&Language::En)),
                Color::Blue,
            )),
            select_style: Style::default().fg(Color::Rgb(255, 112, 67)),
            option_style: Default::default(),
            lang: Default::default(),
            state: Default::default(),
        }
    }
}

#[allow(unused)]
impl<'s> MultiSelect<'s> {
    pub fn new<S>(
        title: String,
        lang: Language,
        options: &Vec<S>,
        option_style: Style,
        icon: Option<&'s str>,
    ) -> Self
    where
        S: ToString,
    {
        let options = options
            .iter()
            .map(|s| Span::styled(s.to_string(), option_style))
            .collect::<Vec<_>>();
        let icon = Span::styled(icon.unwrap_or("?"), Color::Rgb(255, 112, 67));
        let title = Text::from(Line::from_iter(vec![
            icon,
            Span::from(" "),
            Span::from(title).bold(),
        ]));
        let help_msg = Text::from(Line::styled(
            format!("[ {} ]", Common::Help(Help::Select).t(&lang)),
            Color::Blue,
        ));
        let select_style = Style::default().fg(Color::Rgb(255, 112, 67));
        Self {
            options,
            selected: vec![0],
            placeholder: None,
            title,
            help_msg,
            select_style,
            option_style,
            lang,
            state: Default::default(),
        }
    }
    pub fn placeholder(mut self, placeholder: Text<'s>) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
    pub fn selected(mut self, selected: Vec<usize>) -> Self {
        self.selected = selected;
        self
    }
    pub fn help_msg(mut self, help_msg: Text<'s>) -> Self {
        self.help_msg = help_msg;
        self
    }
    pub fn select_style(mut self, select_style: Style) -> Self {
        self.select_style = select_style;
        self
    }
    pub fn render_from(&mut self, area: Rect, frame: &mut Frame) -> () {
        let [title_area, select_area, help_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.options.len() as u16),
            Constraint::Length(1),
        ])
        .areas(area);
        // [title] --------------------------------------------------------------------
        let placeholder_width = if let Some(placeholder) = self.placeholder.as_ref() {
            placeholder.width() as u16
        } else {
            0
        };

        let [title_left_area, title_right_area] = Layout::horizontal([
            Constraint::Length(self.title.width() as u16),
            Constraint::Length(placeholder_width),
        ])
        .spacing(2)
        .areas(title_area);
        // [select] -------------------------------------------------------------------
        let option_layout = Layout::vertical(vec![Constraint::Length(1); self.options.len()]);
        let option_areas = option_layout.split(select_area);
        // [render] ----------------------------------------------------------------------
        frame.render_widget(&self.title, title_left_area);
        if let Some(placeholder) = self.placeholder.as_ref() {
            frame.render_widget(placeholder, title_right_area);
        }
        for (i, option) in self.options.iter().enumerate() {
            let [choose_area, option_line_area] =
                Layout::horizontal([Constraint::Length(8), Constraint::Fill(1)])
                    .areas(option_areas[i]);

            let is_selected = self.selected.contains(&i);
            let option = option.clone();
            if is_selected {
                frame.render_widget(
                    Span::styled(
                        format!("  {} [{}] ", unicode::ARROW_DOUBLE_RIGHT, unicode::CHECK),
                        Color::Rgb(255, 112, 67),
                    ),
                    choose_area,
                );
                frame.render_widget(option.style(self.select_style), option_line_area);
            } else {
                frame.render_widget(
                    Span::styled(format!("    [ ] "), self.option_style),
                    choose_area,
                );
                frame.render_widget(option.style(self.option_style), option_line_area);
            }
        }
        // [help] ----------------------------------------------------------------------
        frame.render_widget(self.help_msg.clone(), help_area);
    }
    pub fn height(&self, w: u16) -> u16 {
        fn handle(w1: u16, w2: u16) -> u16 {
            if w1 > w2 {
                w1 / w2 + 1
            } else {
                1
            }
        }

        let mut height = 0;
        height += self.options.len() as u16;

        let placeholder_width = if let Some(placeholder) = self.placeholder.as_ref() {
            placeholder.width() as u16
        } else {
            0
        };

        height += handle(self.title.width() as u16 + placeholder_width, w);
        height += handle(self.help_msg.width() as u16, w);

        height
    }
}
