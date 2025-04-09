use std::{error::Error, fmt::Display};

use super::terminal::TerminalLogger;

#[derive(Debug, Clone)]
pub enum CheckLogs {
    Welcome,
    Rustc,
    Cargo,
    Git,
    Confirm,
    DependenceNotFound(String),
    DependenceReady(String),
}

impl Display for CheckLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckLogs::Welcome => f.write_str("🥳 Welcome to use ract checker!"),
            CheckLogs::Confirm => f.write_str("🎉 Check finish!"),
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