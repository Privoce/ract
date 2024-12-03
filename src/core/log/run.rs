use std::fmt::Display;

use super::TerminalLogger;

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

const STUDIO_DESC: &str = r#"
🔸 Currently `studio` is only for Makepad
🔸 WASM and GUI are supported
🔸 Default Studio: Which is the studio in gpiler env.toml
"#;

const PROJECT_DESC: &str = r#"
🔸 Now you can run makepad and gen_ui (Comming Soon) project
❗️ Please make sure your project root has a `.gpiler` file to point the project kind
🔸 If you do not know `.gpiler` file, please run `gpiler book` to search (Comming Soon)
"#;

#[derive(Debug)]
pub enum StudioLogs {
    Welcome,
    Desc,
    Gui,
    Stop,
    Error,
}

impl StudioLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}

impl Display for StudioLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudioLogs::Welcome => f.write_str("🥳 Welcome to use gpiler studio!"),
            StudioLogs::Gui => f.write_str("🚀 Start to run the studio in desktop"),

            StudioLogs::Stop => f.write_str("🛑 Stop the studio ..."),
            StudioLogs::Error => f.write_str("❌ Run the studio failed"),
            StudioLogs::Desc => f.write_str(STUDIO_DESC),
        }
    }
}

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
            ProjectLogs::Welcome => f.write_str("🥳 Welcome to use gpiler project runner!"),
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
            output: self.to_string(),
        }
    }
}
