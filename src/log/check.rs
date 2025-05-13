use super::LogExt;
use gen_utils::common::fs;
use rust_i18n::t;
use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug, Clone)]
pub enum CheckLogs {
    Desc,
    Select,
    Found { name: String, path: Option<PathBuf> },
    NotFound(String),
    Complete,
}

impl Display for CheckLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::En).as_ref())
    }
}

impl Error for CheckLogs {}

impl LogExt for CheckLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang = lang.as_str();
        match self {
            CheckLogs::Desc => t!("check.desc", locale = lang),
            CheckLogs::Select => t!("check.select.which", locale = lang),
            CheckLogs::Found { name, path } => {
                if let Some(path) = path {
                    t!(
                        "check.success_path",
                        locale = lang,
                        name = name,
                        path = fs::path_to_str(path)
                    )
                } else {
                    t!("check.success", locale = lang, name = name)
                }
            }
            CheckLogs::NotFound(name) => t!("check.found.failed", locale = lang, name = name),
            CheckLogs::Complete => t!("check.complete", locale = lang),
        }
    }
}
