use super::{LogExt, TerminalLogger};
use rust_i18n::t;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum UninstallLogs {
    Select(String),
    Success(String),
    Failed {
        name: String,
        reason: Option<String>,
    },
}

impl Display for UninstallLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UninstallLogs::Select(name) => {
                f.write_fmt(format_args!("Are you sure to uninstall {}?", name))
            }
            UninstallLogs::Success(name) => {
                f.write_fmt(format_args!("Uninstall {} success!", name))
            }
            UninstallLogs::Failed { name, reason } => f.write_fmt(format_args!(
                "Uninstall {} failed! Reason: {}",
                name,
                reason.as_ref().unwrap_or(&"-".to_string())
            )),
        }
    }
}

impl UninstallLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl Error for UninstallLogs {}

impl LogExt for UninstallLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            UninstallLogs::Select(name) => t!("uninstall.select", locale = lang_str, name = name),
            UninstallLogs::Success(name) => t!("uninstall.success", locale = lang_str, name = name),
            UninstallLogs::Failed { name, reason } => t!(
                "uninstall.failed",
                locale = lang_str,
                name = name,
                reason = reason.as_ref().unwrap_or(&"-".to_string())
            ),
        }
    }
}
