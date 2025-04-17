use std::path::PathBuf;

use gen_utils::common::fs;
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::{
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

impl<'a> From<&CheckItem> for ListItem<'a> {
    fn from(value: &CheckItem) -> Self {
        let (state, color) = if value.state {
            ("🟢 ", Color::Green)
        } else {
            ("🔴 ", Color::Red)
        };

        let mut lines = vec![Line::from_iter(vec![
            Span::from(state),
            Span::styled(value.name.to_string(), color).bold(),
        ])];

        if let Some(path) = value.path.as_ref() {
            lines.push(Line::from(vec![Span::styled(
                fs::path_to_str(path),
                Style::default().add_modifier(Modifier::UNDERLINED),
            )]));
        }

        lines.push(Line::from(""));

        ListItem::new(Text::from(lines))
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
