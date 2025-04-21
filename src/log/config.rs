use std::{error::Error, fmt::Display};
use rust_i18n::t;
use super::{LogExt, TerminalLogger};

#[derive(Debug)]
pub enum ConfigLogs {
    Select,
    LoadSuccess,
    Welcome,
    Desc,
    EnvFail,
    Confirm
}

impl Display for ConfigLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLogs::Select => f.write_str("🔸 Which env file do you want to config?"),
            ConfigLogs::LoadSuccess => f.write_str("Load data success"),
            ConfigLogs::Welcome => f.write_str("🥳 Welcome to use ract config!"),
            ConfigLogs::Desc => f.write_str(DESC),
            ConfigLogs::Confirm => f.write_str("🎉 Config finish!"),
            ConfigLogs::EnvFail => f.write_str("🚫 Config env fail!"),
        }
    }
}

impl ConfigLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl Error for ConfigLogs {}

impl LogExt for ConfigLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            ConfigLogs::Select => t!("config.select.which", locale = lang_str),
            ConfigLogs::LoadSuccess => t!("config.load.success", locale = lang_str),
            ConfigLogs::Welcome => todo!(),
            ConfigLogs::Desc => t!("config.select.desc", locale = lang_str),
            ConfigLogs::EnvFail => todo!(),
            ConfigLogs::Confirm => todo!(),
        }
    }
}

const DESC: &str = r#"
🔸 env: Set the `path` for the chain env.toml file
🔸 chain_env_toml: Set the rust dependency for GenUI toolchain
"#;