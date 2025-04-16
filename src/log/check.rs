use std::{error::Error, fmt::Display};
use rust_i18n::t;
use super::{terminal::TerminalLogger, LogExt};

#[derive(Debug, Clone)]
pub enum CheckLogs {
    Select,
    SelectFailed,
    Welcome,
    Rustc,
    Cargo,
    Git,
    Complete,
    DependenceNotFound(String),
    DependenceReady(String),
}

impl Display for CheckLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckLogs::Select => f.write_str("🔍 Which Option do you want to check?"),
            CheckLogs::SelectFailed => f.write_str("❗️ Select failed!"),
            CheckLogs::Welcome => f.write_str("🥳 Welcome to use ract checker!"),
            CheckLogs::Complete => f.write_str("🎉 Check finish!"),
            CheckLogs::Rustc => f.write_str("✅ rustc is ready!"),
            CheckLogs::Cargo => f.write_str("✅ cargo is ready!"),
            CheckLogs::Git => f.write_str("✅ git is ready!"),
            CheckLogs::DependenceNotFound(name) => f.write_fmt(format_args!(
                "❗️ {} is not found, you can run `ract install` to install in default chain path or run `ract config` to set the path",
                name
            )),
            CheckLogs::DependenceReady(dep) => f.write_fmt(format_args!("✅ {} is ready!", dep)),
        }
    }
}

impl CheckLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl Error for CheckLogs {}

impl LogExt for CheckLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang = lang.as_str();
        match self{
            CheckLogs::Select => t!("check.select.which", locale = lang),
            CheckLogs::SelectFailed => t!("check.select.failed", locale = lang),
            CheckLogs::Welcome => todo!(),
            CheckLogs::Rustc => todo!(),
            CheckLogs::Cargo => todo!(),
            CheckLogs::Git => todo!(),
            CheckLogs::Complete => t!("check.complete", locale = lang),
            CheckLogs::DependenceNotFound(_) => todo!(),
            CheckLogs::DependenceReady(_) => todo!(),
        }
    }
}