use std::{error::Error, fmt::Display};

use rust_i18n::t;

use super::{LogExt};

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
    MakepadStudio,
    CargoMakepadErr,
    MakepadAndroid,
    XCodeConfErr,
    XCodeSelectErr,
    MakepadIos,
    MakepadWasm,
    MakepadWaitInstall,
    MakepadHelp
}

impl Display for InstallLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(crate::entry::Language::En).as_ref())
    }
}

impl LogExt for InstallLogs {
    fn t(&self, lang: crate::entry::Language) -> std::borrow::Cow<str> {
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
            InstallLogs::MakepadStudio => t!("install.makepad_studio", locale = lang),
            InstallLogs::CargoMakepadErr => t!("install.cargo_makepad_err", locale = lang),
            InstallLogs::MakepadAndroid => t!("install.makepad_android", locale = lang),
            InstallLogs::XCodeConfErr => t!("install.xcode_conf_err", locale = lang),
            InstallLogs::MakepadIos => t!("install.makepad_ios", locale = lang),
            InstallLogs::XCodeSelectErr => t!("install.xcode_select_err", locale = lang),
            InstallLogs::MakepadWasm => t!("install.makepad_wasm", locale = lang),
            InstallLogs::MakepadWaitInstall => t!("install.makepad_wait_install", locale = lang),
            InstallLogs::MakepadHelp => t!("install.makepad_help", locale = lang),
        }
    }
}

impl Error for InstallLogs {}
