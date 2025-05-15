use std::{error::Error, fmt::Display};

use rust_i18n::t;

use super::LogExt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RunLogs {
    Studio(StudioLogs),
    Project(ProjectLogs),
}

impl Display for RunLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunLogs::Studio(log) => log.fmt(f),
            RunLogs::Project(log) => log.fmt(f),
        }
    }
}

impl From<StudioLogs> for RunLogs {
    fn from(log: StudioLogs) -> Self {
        RunLogs::Studio(log)
    }
}

impl From<ProjectLogs> for RunLogs {
    fn from(log: ProjectLogs) -> Self {
        RunLogs::Project(log)
    }
}

impl Error for RunLogs {}

impl LogExt for RunLogs {
    fn t(&self, lang: crate::entry::Language) -> std::borrow::Cow<str> {
        match self {
            RunLogs::Studio(log) => log.t(lang),
            RunLogs::Project(log) => log.t(lang),
        }
    }
}

#[derive(Debug)]
pub enum StudioLogs {
    Desc,
    Check,
    Gui,
    Stop,
    Error(String),
    Select,
    Placeholder,
    Custom(String),
}

impl LogExt for StudioLogs {
    fn t(&self, lang: crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            StudioLogs::Desc => t!("studio.desc", locale = lang_str),
            StudioLogs::Check => t!("studio.check", locale = lang_str),
            StudioLogs::Gui => t!("studio.gui", locale = lang_str),
            StudioLogs::Stop => t!("studio.stop", locale = lang_str),
            StudioLogs::Error(reason) => t!("studio.error", locale = lang_str, reason = reason),
            StudioLogs::Select => t!("studio.select", locale = lang_str),
            StudioLogs::Placeholder => t!("studio.placeholder", locale = lang_str),
            StudioLogs::Custom(path) => t!("studio.custom", locale = lang_str, path = path),
        }
    }
}

impl Display for StudioLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(crate::entry::Language::default()).as_ref())
    }
}

impl Error for StudioLogs {}

#[derive(Debug)]
pub enum ProjectLogs {
    Desc,
    Start,
    Stop,
    Error(String),
}

impl Display for ProjectLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(crate::entry::Language::En).as_ref())
    }
}

impl Error for ProjectLogs {}

impl LogExt for ProjectLogs {
    fn t(&self, lang: crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            ProjectLogs::Desc => t!("project.desc", locale = lang_str),
            ProjectLogs::Start => t!("project.start", locale = lang_str),
            ProjectLogs::Stop => t!("project.stop", locale = lang_str),
            ProjectLogs::Error(reason) => t!("project.err", locale = lang_str, reason = reason),
        }
    }
}
