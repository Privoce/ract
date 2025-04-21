use std::path::{Path, PathBuf};

use gen_utils::common::fs;
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::{
    app::unicode,
    entry::Language,
    log::{CheckLogs, LogExt, LogItem},
};

#[derive(Debug, Clone, Default)]
pub struct CheckItem {
    pub name: String,
    pub path: Option<PathBuf>,
    pub state: bool,
}

impl From<(&CheckItem, &Language)> for LogItem {
    fn from(value: (&CheckItem, &Language)) -> Self {
        let (item, lang) = value;
        if item.state {
            LogItem::success(
                CheckLogs::Found {
                    name: item.name.to_string(),
                    path: item.path.clone(),
                }
                .t(lang)
                .to_string(),
            )
        } else {
            LogItem::error(
                CheckLogs::NotFound(item.name.to_string())
                    .t(lang)
                    .to_string(),
            )
        }
    }
}

impl From<Result<PathBuf, which::Error>> for CheckItem {
    fn from(value: Result<PathBuf, which::Error>) -> Self {
        let mut item = CheckItem::default();
        match value {
            Ok(path) => {
                item.path.replace(path);
                item.state = true;
            }
            Err(_) => {
                item.state = false;
            }
        }
        item
    }
}

#[allow(unused)]
impl CheckItem {
    pub fn success<P>(name: String, path: Option<P>) -> Self
    where
        P: AsRef<Path>,
    {
        Self::new(name, path.map(|p| p.as_ref().to_path_buf()), true)
    }
    pub fn error(name: String) -> Self {
        Self::new(name, None, false)
    }
    pub fn new(name: String, path: Option<PathBuf>, state: bool) -> Self {
        Self { name, path, state }
    }
    pub fn draw_list(&self, is_end: bool) -> ListItem {
        let color = if self.state { Color::Green } else { Color::Red };

        let mut lines = vec![Line::from_iter(vec![
            Span::styled(format!("{} ", unicode::CIRCLE_DOT), color),
            Span::from(self.name.to_string()).bold(),
        ])];

        if let Some(path) = self.path.as_ref() {
            lines.push(Line::from(vec![Span::styled(
                fs::path_to_str(path),
                Style::default().add_modifier(Modifier::UNDERLINED),
            )]));
        }

        if !is_end {
            lines.push(Line::from(""));
        }

        ListItem::new(Text::from(lines))
    }
}
