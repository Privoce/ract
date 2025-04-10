use std::{error::Error, fmt::Display};

use super::TerminalLogger;

#[derive(Debug)]
pub enum ConfigLogs {
    Welcome,
    Desc,
    EnvFail,
    Confirm
}

impl Display for ConfigLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

const DESC: &str = r#"
🔸 env: Set the `path` for the chain env.toml file
🔸 chain_env_toml: Set the rust dependency for GenUI toolchain
"#;