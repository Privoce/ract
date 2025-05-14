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
    cell::RefCell,
    fmt::Display,
    sync::mpsc::{Receiver, Sender},
};

pub use add::AddLogs;
pub use check::CheckLogs;
use chrono::{DateTime, Local};
use colored::Colorize;
pub use config::ConfigLogs;
pub use create::CreateLogs;
use gen_utils::common::string::FixedString;
pub use init::InitLogs;
pub use install::InstallLogs;
pub use level::LogLevel;
pub use package::PackageLogs;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
};
pub use run::{ProjectLogs, StudioLogs};
use rust_i18n::t;
pub use terminal::TerminalLogger;
pub use uninstall::UninstallLogs;
pub use wasm::WasmLogs;

use crate::cli::command::Commands;

use super::entry::Language;

pub trait LogExt {
    // use i18n to translate the log message
    fn t(&self, lang: &Language) -> Cow<str>;
    fn terminal(&self, lang: &crate::entry::Language) -> TerminalLogger {
        TerminalLogger {
            output: self.t(lang),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogItem {
    level: LogLevel,
    msg: String,
    /// The datetime of the log （use `chrono` crate）
    datetime: DateTime<Local>,
    is_success: bool,
    /// set fmt as multi line
    multi: bool,
}

impl LogItem {
    /// ## print log item use format (alias of `log` fn)
    #[allow(unused)]
    pub fn print(&self) -> () {
        self.log();
    }
    /// ## print log item
    /// display as:
    /// ```
    /// Ract [${fmt_date_time}]: [${level}] >>> ${msg}
    /// ```
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
            self.msg.split_fixed("\n").into_iter().for_each(|item| {
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
            msg,
            datetime: Local::now(),
            is_success: false,
            multi: false,
        }
    }
    pub fn success(msg: String) -> Self {
        Self {
            level: LogLevel::Info,
            msg,
            datetime: Local::now(),
            is_success: true,
            multi: false,
        }
    }
    pub fn error(msg: String) -> Self {
        Self {
            level: LogLevel::Error,
            msg,
            datetime: Local::now(),
            is_success: false,
            multi: false,
        }
    }
    pub fn warning(msg: String) -> Self {
        Self {
            level: LogLevel::Warn,
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
pub enum CommandType {
    Init,
    Check,
    Create,
    Config,
    Studio,
    Install,
    Wasm,
    Run,
    Pkg,
    Add,
    Update,
    Uninstall,
    #[default]
    Unknown,
}

impl From<&Commands> for CommandType {
    fn from(value: &Commands) -> Self {
        match value {
            Commands::Create(_) => CommandType::Create,
            Commands::Check => CommandType::Check,
            Commands::Install => CommandType::Install,
            Commands::Run => CommandType::Run,
            Commands::Init => CommandType::Init,
            Commands::Config => CommandType::Config,
            Commands::Studio => CommandType::Studio,
            Commands::Wasm(_) => CommandType::Wasm,
            Commands::Pkg => CommandType::Pkg,
            Commands::Add { .. } => CommandType::Add,
            Commands::Update(_) => CommandType::Update,
            Commands::Uninstall => CommandType::Uninstall,
        }
    }
}

impl Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CommandType::Init => "INIT",
            CommandType::Check => "CHECK",
            CommandType::Create => "CREATE",
            CommandType::Config => "CONFIG",
            CommandType::Unknown => "UNKNOWN",
            CommandType::Install => "INSTALL",
            CommandType::Studio => "STUDIO",
            CommandType::Wasm => "WASM",
            CommandType::Run => "RUN",
            CommandType::Pkg => "PKG",
            CommandType::Add => "ADD",
            CommandType::Update => "UPDATE",
            CommandType::Uninstall => "UNINSTALL",
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
    #[allow(unused)]
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
    Help(Help),
    Command(Command),
    Fs(Fs),
    TmpStore(String),
    Option(Options),
}

impl LogExt for Common {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang_str = lang.as_str();
        match self {
            Common::Os => t!("common.os", locale = lang_str),
            Common::Version => t!("common.version", locale = lang_str),
            Common::Language => t!("common.language", locale = lang_str),
            Common::Total => t!("common.total", locale = lang_str),
            Common::Help(help) => help.t(lang),
            Common::Command(cmd) => cmd.t(lang),
            Common::Fs(fs) => fs.t(lang),
            Common::TmpStore(value) => t!("common.tmp_store", locale = lang_str, value = value),
            Common::Option(options) => options.t(lang),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
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

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Help {
    Select,
    MultiSelect,
    EditNormal,
    EditComplex,
    Log,
}

impl LogExt for Help {
    fn t(&self, lang: &Language) -> Cow<str> {
        let lang = lang.as_str();
        match self {
            Help::Select => t!("common.help.select", locale = lang),
            Help::MultiSelect => t!("common.help.multi_select", locale = lang),
            Help::EditNormal => t!("common.help.edit.normal", locale = lang),
            Help::EditComplex => t!("common.help.edit.complex", locale = lang),
            Help::Log => t!("common.help.log_tab", locale = lang),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
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

#[allow(unused)]
#[derive(Debug, Clone)]
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

/// # Channel for log item
/// which will be used to send log item to the main thread in component
/// may deprecated in the future, (it not design great)
pub struct ComponentChannel<T> {
    pub sender: Sender<LogItem>,
    pub receiver: Receiver<LogItem>,
    pub run_channel: Option<RunChannel<T>>,
}

impl<T> ComponentChannel<T> {
    pub fn new(run_channel: Option<RunChannel<T>>) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            sender,
            receiver,
            run_channel,
        }
    }
    // pub fn send(&self, item: LogItem) -> () {
    //     self.sender.send(item).unwrap();
    // }
    // pub fn recv(&self) -> LogItem {
    //     self.receiver.recv().unwrap()
    // }
}

pub struct RunChannel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> RunChannel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self { sender, receiver }
    }
}
