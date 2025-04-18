use ratatui::{style::Style, text::{Line, Text}};

pub struct Select<'s> { 
    pub options: Vec<Line<'s>>,
    pub selected: usize,
    pub placeholder: Text<'s>,
    pub title:Text<'s>,
    pub default: Option<usize>,
    pub help_msg: Text<'s>,
    pub select_style: Style,
}


