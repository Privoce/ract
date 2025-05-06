use super::{LogExt, TerminalLogger};
use rust_i18n::t;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum InitLogs {
    Init,
    Complete,
    Chain,
    ChainSuccess,
    ChainFailed(String),
    Env,
    EnvDesc,
    EnvSuccess,
    EnvFailed(String),
}

impl Display for InitLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitLogs::Init => f.write_str("🚀 Start to init ract..."),
            InitLogs::Complete => f.write_str("🎉 Init ract successfully!"),
            InitLogs::Chain => f.write_str("Init chain"),
            InitLogs::EnvDesc => {
                f.write_str("Ract will help you create an .env file for link chains")
            }
            InitLogs::ChainSuccess => f.write_str("✅ Chain init successfully!"),
            InitLogs::Env => f.write_str("Init env"),
            InitLogs::EnvSuccess => f.write_str("✅ Env init successfully!"),
            InitLogs::ChainFailed(reason) => {
                write!(f, "❌ Chain init failed: {}", reason)
            }
            InitLogs::EnvFailed(reason) => {
                write!(f, "❌ Env init failed: {}", reason)
            }
        }
    }
}

impl InitLogs {
    pub fn terminal(&self, lang: &crate::entry::Language) -> TerminalLogger {
        TerminalLogger {
            output: self.t(lang),
        }
    }
}

impl Error for InitLogs {}

impl LogExt for InitLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        match self {
            InitLogs::Init => t!("init.start", locale = lang.as_str()),
            InitLogs::Complete => t!("init.complete", locale = lang.as_str()),
            InitLogs::Chain => t!("init.chain.title", locale = lang.as_str()),
            InitLogs::Env => t!("init.env.title", locale = lang.as_str()),
            InitLogs::EnvDesc => t!("init.env.desc", locale = lang.as_str()),
            InitLogs::ChainSuccess => t!("init.chain.success", locale = lang.as_str()),
            InitLogs::EnvSuccess => t!("init.env.success", locale = lang.as_str()),
            InitLogs::ChainFailed(reason) => {
                t!("init.chain.failed", locale = lang.as_str(), reason = reason)
            }
            InitLogs::EnvFailed(reason) => {
                t!("init.env.failed", locale = lang.as_str(), reason = reason)
            }
        }
    }
}
