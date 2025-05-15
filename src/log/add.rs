use std::{error::Error, fmt::Display};

use rust_i18n::t;

use crate::entry::Language;

use super::{LogExt};

#[derive(Debug, Clone)]
pub enum AddLogs {
    DownloadFailed(String),
    DownloadSuccess(String),
    Downloading(String),
    WriteInTomlFailed(String),
    Complete(String),
}

impl Display for AddLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddLogs::DownloadFailed(reason) => write!(f, "❌ Download plugin failed: {}", reason),
            AddLogs::DownloadSuccess(name) => write!(f, "🎉 Download plugin: {} success", name),
            AddLogs::Downloading(name) => {
                write!(f, "🔸 Downloading plugin: {} . Please wait...", name)
            }
            AddLogs::WriteInTomlFailed(name) => {
                write!(f, "❌ Write plugin: {} in gen_ui.toml failed", name)
            }
            AddLogs::Complete(name) => write!(f, "🎉 Add plugin: {} success!", name),
        }
    }
}

impl Error for AddLogs {}

impl LogExt for AddLogs {
    fn t(&self, lang: Language) -> std::borrow::Cow<str> {
        match self {
            AddLogs::DownloadFailed(reason) => t!(
                "add.download.failed",
                locale = lang.as_str(),
                reason = reason
            ),
            AddLogs::DownloadSuccess(name) => {
                t!("add.download.success", locale = lang.as_str(), name = name)
            }
            AddLogs::Downloading(name) => {
                t!("add.download.waiting", locale = lang.as_str(), name = name)
            }
            AddLogs::WriteInTomlFailed(name) => t!(
                "add.write_in_toml_fail",
                locale = lang.as_str(),
                name = name
            ),
            AddLogs::Complete(name) => t!("add.complete", locale = lang.as_str(), name = name),
        }
    }
}
