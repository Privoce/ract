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
mod uninstall;
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
use gen_utils::common::string::FixedString;
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
pub use uninstall::UninstallLogs;
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
    /// set fmt as multi line
    multi: bool,
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
    pub fn fmt_lines(&self) -> Vec<Line<'static>> {
        let mut fmt: Vec<Span> = vec![
            Span::styled("Ract", Style::default().bold().fg(Color::Rgb(255, 112, 67))),
            Span::styled(self.fmt_timestamp(), Style::default().fg(Color::White)),
            Span::styled(
                format!("[{}]", self.level.fmt_level()),
                Style::default().fg(self.level_color()),
            ),
            Span::styled(" >>> ", Style::default().fg(Color::White)),
        ];

        if self.multi {
            // split msg by '\n'
            let mut res = vec![Line::from(fmt)];
            res.push(Line::raw(""));
            self.msg.split("\n").for_each(|item| {
                res.push(Line::from(Span::styled(
                    item.to_string(),
                    Style::default().fg(Color::White),
                )));
            });

            return res;
        } else {
            fmt.push(Span::styled(
                self.msg.clone(),
                Style::default().fg(Color::White),
            ));
            return vec![Line::from(fmt)];
        }
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
            multi: false,
        }
    }
    pub fn success(msg: String) -> Self {
        Self {
            level: LogLevel::Info,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: true,
            multi: false,
        }
    }
    pub fn error(msg: String) -> Self {
        Self {
            level: LogLevel::Error,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: false,
            multi: false,
        }
    }
    pub fn warning(msg: String) -> Self {
        Self {
            level: LogLevel::Warn,
            ty: Default::default(),
            msg,
            datetime: Local::now(),
            is_success: false,
            multi: false,
        }
    }
    pub fn multi(mut self) -> Self {
        self.multi = true;
        self
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
    pub cache: RefCell<Option<(Text<'static>, u16)>>,
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
        self.draw_text_with_width(100).0
    }
    /// ## draw text with width
    /// which will return Text and line length
    pub fn draw_text_with_width(&self, w: u16) -> (Text, u16) {
        if let Some(text) = self.cache.borrow().as_ref() {
            return text.clone();
        }

        let (items, line_length) =
            self.items
                .iter()
                .fold((Vec::new(), 0_u16), |(mut lines, mut line_length), log| {
                    let line = log.fmt_lines();
                    for item in &line {
                        if item.width() < w as usize {
                            line_length += 1;
                        } else {
                            line_length += (item.width() / (w as usize)) as u16;
                        }
                    }

                    lines.extend(line);
                    (lines, line_length)
                });

        let text = Text::from_iter(items);
        *self.cache.borrow_mut() = Some((text.clone(), line_length));
        (text, line_length)
    }
    pub fn extend(&mut self, items: Vec<LogItem>) -> () {
        self.items.extend(items);
        self.cache.borrow_mut().take();
    }
}

pub enum Common {
    Os,
    Version,
    Language,
    Total,
    Doc,
    Help(Help),
    Command(Command),
    Fs(Fs),
    TmpStore(String),
    Option(Options)
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
            Common::Command(cmd) => cmd.t(lang),
            Common::Fs(fs) => fs.t(lang),
            Common::TmpStore(value) => t!("common.tmp_store", locale = lang_str, value = value),
            Common::Option(options) => options.t(lang),
        }
    }
}

pub enum Options {
    Default,
    Custom,
    Yes,
    No,
}

impl LogExt for Options {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Options::Default => t!("common.option.default", locale = lang),
            Options::Custom => t!("common.option.custom", locale = lang),
            Options::Yes => t!("common.option.yes", locale = lang),
            Options::No => t!("common.option.no", locale = lang),
        }
    }
}

pub enum Help {
    Select,
    EditNormal,
    EditComplex,
}

impl LogExt for Help {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Help::Select => t!("common.help.select", locale = lang),
            Help::EditNormal => t!("common.help.edit.normal", locale = lang),
            Help::EditComplex => t!("common.help.edit.complex", locale = lang),
        }
    }
}

pub enum Command {
    Select,
    Q,
    Wq,
    W,
}

impl Command {
    pub fn options() -> Vec<&'static str> {
        vec!["q", "wq", "w"]
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "q" => Self::Q,
            "wq" => Self::Wq,
            "w" => Self::W,
            _ => unreachable!("Select Component can not be reached!"),
        }
    }
}

impl LogExt for Command {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Command::Select => t!("common.command.select", locale = lang),
            Command::Q => t!("common.command.quit", locale = lang),
            Command::Wq => t!("common.command.write_quit", locale = lang),
            Command::W => t!("common.command.write", locale = lang),
        }
    }
}

pub enum Fs {
    ReadSuccess(String),
    ReadError(String),
    WriteSuccess(String),
    WriteError(String),
}

impl LogExt for Fs {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Fs::ReadSuccess(name) => t!("common.fs.read.success", locale = lang, name = name),
            Fs::ReadError(reason) => t!("common.fs.read.error", locale = lang, reason = reason),
            Fs::WriteSuccess(name) => t!("common.fs.write.success", locale = lang, name = name),
            Fs::WriteError(reason) => t!("common.fs.write.error", locale = lang, reason = reason),
        }
    }
}
