use std::fmt::Display;

use super::TerminalLogger;

#[allow(dead_code)]
#[derive(Debug)]
pub enum InstallLogs {
    Welcome,
    Desc,
    Install(String),
    Installed(String),
    UnInstalled(String),
    InstallErr(String),
    Rustc,
    Cargo,
    Git,
    All,
    Default,
    Confirm(String),
}

impl Display for InstallLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallLogs::Welcome => f.write_str("🥳 Welcome to use ract Install!"),
            InstallLogs::Rustc => f.write_str("✅ Rustc has been installed successfully!"),
            InstallLogs::Cargo => f.write_str("✅ Cargo has been installed successfully!"),
            InstallLogs::Git => f.write_str("✅ Git has been installed successfully!"),
            InstallLogs::All => {
                f.write_str("✅ All dependencies have been installed successfully!")
            }
            InstallLogs::Default => {
                f.write_str("✅ Default dependencies have been installed successfully!")
            }
            InstallLogs::Confirm(t) => f.write_fmt(format_args!("🎉 Install {} finish!", t)),
            InstallLogs::Desc => f.write_str(DESC),
            InstallLogs::Install(t) => f.write_fmt(format_args!("🚀 Start to install: {} ...", t)),
            InstallLogs::InstallErr(t) => f.write_fmt(format_args!("❌ Install {} failed!", t)),
            InstallLogs::Installed(t) => f.write_fmt(format_args!("✅ {} has been installed!", t)),
            InstallLogs::UnInstalled(t) => {
                f.write_fmt(format_args!("❌ {} has not been installed!", t))
            }
        }
    }
}

impl InstallLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}

const DESC: &str = r#"
🔸 Rustc: Install the rustc compiler
🔸 Cargo: Install the cargo package manager
🔸 Git: Install the git version control system
🔸 All: Install all dependencies (include: [Rustc, Cargo, Git, All_Underlayer])
🔸 Default: Install default dependencies (include: [Rustc, Cargo, Git, Makepad_Underlayer])
"#;
