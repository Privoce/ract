use std::{
    cell::OnceCell,
    fmt::{format, Display},
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Span, Text},
    Frame,
};

use super::unicode;

#[derive(Debug, Clone, Default)]
pub struct Tab<'a> {
    pub direction: Direction,
    pub tabs: Vec<&'a str>,
    pub selected: usize,
    pub selected_style: Style,
}

impl<'a> Tab<'a> {
    pub fn new(tabs: Vec<&'a str>) -> Self {
        Self {
            direction: Default::default(),
            tabs,
            selected: 0,
            selected_style: Style::default(),
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }
    /// ## Render the tabs
    /// ### Vertical
    /// ```text
    /// ${tab} ...
    /// ------ ...
    /// ${pane}
    /// ```
    /// ### Horizontal
    /// ```text
    /// ${tab} | ${pane}
    /// ...    |
    /// ...    |
    /// ```
    pub fn render<R>(&self, area: Rect, frame: &mut Frame, render_pane: R)
    where
        R: FnOnce(Rect, &mut Frame),
    {
        let pane_area = match self.direction {
            Direction::Horizontal => {
                // if tab is not length as area.height, do fill " "
                let mut tabs = self.tabs.clone();
                for _ in self.tabs.len()..area.height as usize {
                    tabs.push(" ");
                }

                let (tab_constraints, tab_lens) =
                    tabs.iter()
                        .fold((Vec::new(), Vec::new()), |(mut cons, mut lens), tab| {
                            let len = tab.len() as u16;
                            cons.push(Constraint::Length(1));
                            lens.push(len as usize);
                            (cons, lens)
                        });
                let max = tab_lens.into_iter().max().unwrap_or(0) + 3;
                let [tab_area, pane_area] = Layout::horizontal([
                    Constraint::Length(max as u16),
                    Constraint::Percentage(100),
                ])
                .spacing(1)
                .areas(area);
                // [tabs] -------------------------------------------------------------------------------------
                let tab_areas = Layout::vertical(tab_constraints).split(tab_area);
                for (i, tab) in tabs.iter().enumerate() {
                    let [tab_area, line_area] =
                        Layout::horizontal([Constraint::Length(max as u16), Constraint::Length(1)])
                            .areas(tab_areas[i]);
                    let (l, r, style) = if i == self.selected {
                        (unicode::ARROW_RIGHT_SHARP, " ", self.selected_style)
                    } else {
                        (" ", " ", Style::default())
                    };
                    let [select_line_area] =
                        Layout::vertical([Constraint::Length(1)]).areas(line_area);
                    frame.render_widget(Span::styled(unicode::LINE_V, style), select_line_area);
                    frame.render_widget(
                        Span::styled(format!("{}{}{}", l, tab, r.repeat(max - tab.len())), style),
                        tab_area,
                    );
                }

                pane_area
            }
            Direction::Vertical => {
                let tab_constraints = self
                    .tabs
                    .iter()
                    .map(|tab| Constraint::Length(tab.len() as u16 + 2))
                    .collect::<Vec<_>>();
                let [tab_area, line_area, pane_area] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Percentage(100),
                ])
                .areas(area);
                // [tabs] -------------------------------------------------------------------------------------
                let tab_areas = Layout::horizontal(tab_constraints)
                    .spacing(1)
                    .split(tab_area);
                frame.render_widget(
                    Span::styled(
                        unicode::LINE_H.repeat(line_area.width as usize),
                        Style::default(),
                    ),
                    line_area,
                );
                for (i, tab) in self.tabs.iter().enumerate() {
                    let line_len = tab.len() as u16 + 2;
                    let (l, r, style) = if i == self.selected {
                        let [select_line_area] =
                            Layout::horizontal([Constraint::Length(line_len)]).areas(line_area);
                        frame.render_widget(
                            Span::styled(
                                unicode::LINE_H.repeat(line_len as usize),
                                self.selected_style,
                            ),
                            select_line_area,
                        );
                        (unicode::ARROW_RIGHT_SHARP, " ", self.selected_style)
                    } else {
                        (" ", " ", Style::default())
                    };
                    frame.render_widget(
                        Span::styled(format!("{}{}{}", l, tab, r), style),
                        tab_areas[i],
                    );
                }

                pane_area
            }
        };
        render_pane(pane_area, frame);
    }
}
