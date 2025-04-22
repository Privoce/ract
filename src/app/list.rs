use std::collections::HashMap;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::unicode;

/// # KV
/// Key-Value item for selectable, used in editable list
/// ## UI
/// ```
/// ${icon} ${key} » ${value}
/// // such as
/// ➤ name » John
/// ```
pub struct KV<'k> {
    pub icon: Span<'k>,
    pub selected: bool,
    pub key: Span<'k>,
    pub value: Span<'k>,
    /// style when selected
    pub style: Style,
}

impl<'k> KV<'k> {
    pub fn new(key: String, value: String) -> Self {
        Self {
            icon: Span::from(unicode::ARROW_RIGHT_SHARP),
            selected: false,
            key: Span::from(key),
            value: Span::from(value),
            style: Style::default().fg(Color::Rgb(255, 112, 67)),
        }
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn icon(mut self, icon: &'k str) -> Self {
        self.icon = Span::from(icon);
        self
    }
}

impl<'k> From<KV<'k>> for Line<'k> {
    fn from(kv: KV<'k>) -> Self {
        let (icon, style) = if kv.selected {
            (kv.icon.style(kv.style), kv.style)
        } else {
            (Span::from("\r"), Style::default())
        };

        Line::from_iter([
            icon,
            Span::from(" "),
            kv.key.style(style),
            Span::from(format!(" {} ", unicode::ARROW_DOUBLE_RIGHT)),
            kv.value,
        ])
    }
}
