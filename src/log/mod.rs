mod add;
mod check;
pub mod compiler;
mod config;
mod create;
pub mod error;
mod init;
mod install;
mod level;
mod package;
mod run;
mod terminal;
mod wasm;

use std::{
    borrow::Cow,
    cell::{OnceCell, RefCell},
    fmt::Display,
};

pub use add::AddLogs;
pub use check::CheckLogs;
use chrono::{DateTime, Local};
use colored::Colorize;
use compiler::CompilerLogs;
pub use config::ConfigLogs;
pub use create::CreateLogs;
pub use init::InitLogs;
pub use install::InstallLogs;
pub use level::LogLevel;
pub use package::PackageLogs;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text, ToLine},
};
pub use run::{ProjectLogs, RunLogs, StudioLogs};
use rust_i18n::t;
pub use terminal::TerminalLogger;
pub use wasm::WasmLogs;

use super::entry::Language;

pub trait LogExt {
    // use i18n to translate the log message
    fn t(&self, lang: &Language) -> Cow<str>;
}

#[derive(Debug, Clone)]
pub struct LogItem {
    level: LogLevel,
    ty: LogType,
    msg: String,
    /// The datetime of the log （use `chrono` crate）
    datetime: DateTime<Local>,
    is_success: bool,
}

impl LogItem {
    pub fn log(&self) -> () {
        println!(
            "{}{}[{}] >>> {}",
            "Ract".truecolor(255, 112, 67).bold(),
            self.fmt_timestamp(),
            self.level.colorize(),
            self.msg
        );
    }
    /// ## fmt as ratatui text line for colorful display
    /// display as:
    /// Ract [${fmt_date_time}]: [${level}] >>> ${msg}
    pub fn fmt_line(&self) -> Line<'static> {
        Line::from(vec![
            Span::styled("Ract", Style::default().bold().fg(Color::Rgb(255, 112, 67))).into(),
            Span::styled(self.fmt_timestamp(), Style::default().fg(Color::White)).into(),
            Span::styled(
                format!("[{}]", self.level.fmt_level()),
                Style::default().fg(self.level_color()),
            )
            .into(),
            Span::styled(" >>> ", Style::default().fg(Color::White)).into(),
            Span::styled(self.msg.clone(), Style::default().fg(Color::White)).into(),
        ])
    }
    fn level_color(&self) -> Color {
        if self.is_success {
            Color::Green
        } else {
            self.level.color()
        }
    }
    pub fn fmt_timestamp(&self) -> String {
        self.datetime.format(" [%Y-%m-%d %H:%M:%S] ").to_string()
    }

    pub fn info(msg: String) -> Self {
        Self {
            level: LogLevel::Info,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: false,
        }
    }
    pub fn success(msg: String) -> Self {
        Self {
            level: LogLevel::Info,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: true,
        }
    }
    pub fn error(msg: String) -> Self {
        Self {
            level: LogLevel::Error,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum LogType {
    Init,
    Check,
    Create,
    Config,
    #[default]
    Unknown,
}

impl LogType {
    pub fn is_unknown(&self) -> bool {
        matches!(self, LogType::Unknown)
    }
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogType::Init => "INIT",
            LogType::Check => "CHECK",
            LogType::Create => "CREATE",
            LogType::Config => "CONFIG",
            LogType::Unknown => "UNKNOWN",
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Log {
    pub items: Vec<LogItem>,
    pub cache: RefCell<Option<Text<'static>>>,
}

impl Log {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, item: LogItem) -> () {
        self.items.push(item);
        self.cache.borrow_mut().take();
    }
    pub fn iter(&self) -> impl Iterator<Item = &LogItem> {
        self.items.iter()
    }
    pub fn draw_text(&self) -> Text {
        if let Some(text) = self.cache.borrow().as_ref() {
            return text.clone();
        }
        let items: Vec<Line> = self.items.iter().map(|log| log.fmt_line()).collect();
        let text = Text::from_iter(items);
        *self.cache.borrow_mut() = Some(text.clone());
        text
    }
    pub fn extend(&mut self, items: Vec<LogItem>) -> () {
        self.items.extend(items);
        self.cache.borrow_mut().take();
    }
}

pub enum Common{
    Os,
    Version,
    Language,
    Total,
    Doc,
    Help(Help)
}

impl LogExt for Common {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang_str = lang.as_str();
        match self {
            Common::Os => t!("common.os", locale = lang_str),
            Common::Version => t!("common.version", locale = lang_str),
            Common::Language => t!("common.language", locale = lang_str),
            Common::Total => t!("common.total", locale = lang_str),
            Common::Doc => t!("common.doc", locale = lang_str),
            Common::Help(help) => help.t(lang),
        }
    }
}

pub enum Help {
    Select
}

impl LogExt for Help {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Help::Select => t!("common.help.select", locale = lang),
        }
    }
}