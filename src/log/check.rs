use super::{terminal::TerminalLogger, LogExt};
use gen_utils::common::fs;
use rust_i18n::t;
use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug, Clone)]
pub enum CheckLogs {
    Desc,
    Select,
    SelectFailed,
    Found { name: String, path: Option<PathBuf> },
    NotFound(String),
    Complete,
}

impl Display for CheckLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::En).as_ref())
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
        match self {
            CheckLogs::Desc => t!("check.desc", locale = lang),
            CheckLogs::Select => t!("check.select.which", locale = lang),
            CheckLogs::SelectFailed => t!("check.select.failed", locale = lang),
            CheckLogs::Found { name, .. } => t!("check.found.success", locale = lang, name = name),
            CheckLogs::NotFound(name) => t!("check.found.failed", locale = lang, name = name),
            CheckLogs::Complete => t!("check.complete", locale = lang),
        }
    }
}
