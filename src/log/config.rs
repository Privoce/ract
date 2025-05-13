use super::LogExt;
use rust_i18n::t;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConfigLogs {
    LoadSuccess,
    Desc,
}

impl Display for ConfigLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::En).as_ref())
    }
}

impl Error for ConfigLogs {}

impl LogExt for ConfigLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            ConfigLogs::LoadSuccess => t!("config.load.success", locale = lang_str),
            ConfigLogs::Desc => t!("config.desc", locale = lang_str),
        }
    }
}
