use std::{error::Error, fmt::Display};

use rust_i18n::t;

use super::{LogExt, TerminalLogger};

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

const PROJECT_DESC: &str = r#"
🔸 Now you can run makepad and gen_ui (Comming Soon) project
❗️ Please make sure your project root has a `.ract` file to point the project kind
🔸 If you do not know `.ract` file, please run `ract book` to search (Comming Soon)
"#;

#[derive(Debug)]
pub enum StudioLogs {
    Desc,
    Check,
    Gui,
    Stop,
    Error(String),
}

impl StudioLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl LogExt for StudioLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            StudioLogs::Desc => t!("studio.desc", locale = lang_str),
            StudioLogs::Check => t!("studio.check", locale = lang_str),
            StudioLogs::Gui => t!("studio.gui", locale = lang_str),
            StudioLogs::Stop => t!("studio.stop", locale = lang_str),
            StudioLogs::Error(reason) => t!("studio.error", locale = lang_str, reason = reason),
        }
    }
}

impl Display for StudioLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::default()).as_ref())
    }
}

impl Error for StudioLogs {}

#[derive(Debug)]
pub enum ProjectLogs {
    Welcome,
    Desc,
    Start,
    Stop,
    Error(String),
}

impl Display for ProjectLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectLogs::Welcome => f.write_str("🥳 Welcome to use ract project runner!"),
            ProjectLogs::Start => f.write_str("🚀 Start to run the project ..."),
            ProjectLogs::Stop => f.write_str("🛑 Stop the project ..."),
            ProjectLogs::Error(t) => f.write_fmt(format_args!("❌ Run the project failed: {}", t)),
            ProjectLogs::Desc => f.write_str(PROJECT_DESC),
        }
    }
}

impl ProjectLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl Error for ProjectLogs {}
