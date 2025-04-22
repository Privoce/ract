use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Paragraph, Wrap},
    Frame,
};

/// # Input
/// ```
///  ┌─────────────────────────────────┐
///  │${value}/${placeholder} │        │         
///  └─────────────────────────────────┘
/// ```
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub mode: InputMode,
    pub value: String,
    pub placeholder: String,
    pub char_index: usize,
    pub style: Style,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self.move_cursor_end();
        self
    }
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn char_index(mut self, char_index: usize) -> Self {
        let byte_length = self.byte_length();
        if byte_length != 0 && byte_length <= char_index {
            self.char_index = char_index;
        }
        self
    }
    pub fn reset(&mut self) {
        self.value.clear();
        self.char_index = 0;
    }
    pub fn move_cursor(&mut self, offset: isize) {
        let new_index = (self.char_index as isize + offset).max(0) as usize;
        self.char_index = new_index.min(self.byte_length());
    }
    pub fn move_cursor_end(&mut self) {
        self.char_index = self.byte_length();
    }
    // get the real byte length of the string
    pub fn byte_length(&self) -> usize {
        self.value.chars().count()
    }
    pub fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.value.len())
    }

    pub fn input(&mut self, s: &str) {
        let byte_index = self.byte_index();
        // change s -> chars then do insert
        for (index, char) in s.char_indices() {
            self.value.insert(byte_index + index, char);
        }
        // update the char index
        self.char_index += s.chars().count();
    }
    pub fn delete(&mut self) {
        if self.char_index == 0 {
            return;
        } else {
            let deleted_char_index = self.char_index - 1;
            let left_chars = self.value.chars().take(deleted_char_index);
            let right_chars = self.value.chars().skip(deleted_char_index + 1);
            self.value = left_chars.chain(right_chars).collect();
            self.move_cursor(-1);
        }
    }
    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let text = if self.value.is_empty() {
            Span::styled(&self.placeholder, Color::Rgb(51, 51, 51))
        } else {
            Span::styled(&self.value, Style::default())
        };

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().border_type(BorderType::Rounded))
                .wrap(Wrap { trim: true })
                .scroll((1, 0)),
            area,
        );
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputMode {
    Edit,
    #[default]
    Normal,
}

impl InputMode {
    pub fn next(&mut self) -> () {
        match self {
            InputMode::Edit => {
                *self = InputMode::Normal;
            }
            InputMode::Normal => {
                *self = InputMode::Edit;
            }
        }
    }

    pub fn is_edit(&self) -> bool {
        matches!(self, InputMode::Edit)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, InputMode::Normal)
    }
}

#[cfg(test)]
mod input_test {
    #[test]
    fn test1() {
        let mut input = super::Input::new()
            .value("你好!".to_string())
            .mode(super::InputMode::Edit);

        input.input("123");
        input.delete();
        input.input("赛道");
        input.delete();

        dbg!(input.value);
        dbg!(input.char_index);
    }
}
