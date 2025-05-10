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
        // match self {
        //     // InstallLogs::Welcome => f.write_str("🥳 Welcome to use ract Install!"),
        //     InstallLogs::Rustc => f.write_str("✅ Rustc has been installed successfully!"),
        //     InstallLogs::Cargo => f.write_str("✅ Cargo has been installed successfully!"),
        //     InstallLogs::Git => f.write_str("✅ Git has been installed successfully!"),
        //     InstallLogs::All => {
        //         f.write_str("✅ All dependencies have been installed successfully!")
        //     }
        //     InstallLogs::Default => {
        //         f.write_str("✅ Default dependencies have been installed successfully!")
        //     }
        //     InstallLogs::Confirm(t) => f.write_fmt(format_args!("🎉 Install {} finish!", t)),
        //     InstallLogs::Desc => f.write_str(DESC),
        //     InstallLogs::Install(t) => f.write_fmt(format_args!("🚀 Start to install: {} ...", t)),
        //     InstallLogs::InstallErr(t) => f.write_fmt(format_args!("❌ Install {} failed!", t)),
        //     InstallLogs::Installed(t) => f.write_fmt(format_args!("✅ {} has been installed!", t)),
        //     InstallLogs::UnInstalled(t) => {
        //         f.write_fmt(format_args!("❌ {} has not been installed!", t))
        //     }
        //     InstallLogs::Check { current, num, total } => 
        //         f.write_fmt(format_args!(
        //             "🔸 Check: {} ({}/{})",
        //             current, num, total
        //         )),
        //     InstallLogs::CheckTitle => f.write_str("🔸 Check:"),
        // }
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
            InstallLogs::Confirm(_) => todo!(),
        }
    }
}

impl Error for InstallLogs {}

const DESC: &str = r#"
🔸 Rustc: Install the rustc compiler
🔸 Cargo: Install the cargo package manager
🔸 Git: Install the git version control system
🔸 All: Install all dependencies (include: [Rustc, Cargo, Git, All_Underlayer])
🔸 Default: Install default dependencies (include: [Rustc, Cargo, Git, Makepad_Underlayer])
"#;
