use super::LogExt;
use rust_i18n::t;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum CreateLogs {
    Workspace,
    Unsupported,
    Git,
    GitErr,
    Cargo,
    Confirm,
    Cancel,
    CargoErr,
}

impl Display for CreateLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(crate::entry::Language::En).as_ref())
    }
}

impl Error for CreateLogs {}

impl LogExt for CreateLogs {
    fn t(&self, lang: crate::entry::Language) -> std::borrow::Cow<str> {
        let lang = lang.as_str();
        match self {
            CreateLogs::Workspace => t!("create.workspace", locale = lang),
            CreateLogs::Unsupported => t!("create.unsupported", locale = lang),
            CreateLogs::Git => t!("create.git", locale = lang),
            CreateLogs::GitErr => t!("create.git_err", locale = lang),
            CreateLogs::Cargo => t!("create.cargo", locale = lang),
            CreateLogs::Confirm => t!("create.confirm", locale = lang),
            CreateLogs::Cancel => t!("create.cancel", locale = lang),
            CreateLogs::CargoErr => t!("create.cargo_err", locale = lang),
        }
    }
}
