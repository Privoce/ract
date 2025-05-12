use std::{error::Error, fmt::Display};

use rust_i18n::t;

use super::{LogExt, TerminalLogger};

#[allow(dead_code)]
#[derive(Debug)]
pub enum InstallLogs {
    Desc,
    Check { current: String, num: u8, total: u8 },
    CheckTitle,
    Select,
    Install(String),
    Installed(String),
    UnInstalled(String),
    InstallErr(String),
    Confirm(String),
}

impl Display for InstallLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::En).as_ref())
    }
}

impl InstallLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl LogExt for InstallLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang = lang.as_str();

        match self {
            InstallLogs::Desc => t!("install.desc", locale = lang),
            InstallLogs::Check {
                current,
                num,
                total,
            } => t!(
                "install.check",
                locale = lang,
                current = current,
                num = num,
                total = total
            ),
            InstallLogs::CheckTitle => t!("install.check_title", locale = lang),
            InstallLogs::Select => t!("install.select", locale = lang),
            InstallLogs::Install(name) => t!("install.install", locale = lang, name = name),
            InstallLogs::Installed(name) => t!("install.installed", locale = lang, name = name),
            InstallLogs::UnInstalled(name) => t!("install.uninstalled", locale = lang, name = name),
            InstallLogs::InstallErr(name) => t!("install.install_err", locale = lang, name = name),
            InstallLogs::Confirm(name) => t!("install.confirm", locale = lang, name = name),
        }
    }
}

impl Error for InstallLogs {}
