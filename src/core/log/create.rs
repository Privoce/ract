use std::{error::Error, fmt::Display};

use super::terminal::TerminalLogger;

#[derive(Debug, Clone, Copy)]
pub enum CreateLogs {
    Welcome,
    Workspace,
    Git,
    GitErr,
    Cargo,
    Confirm,
    Cancel,
    CargoErr
}

impl Display for CreateLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateLogs::Welcome => {
                f.write_str("❤️ WELOCME TO GENUI, ract is a build tool for you!")
            }
            CreateLogs::Confirm => f.write_str("🎉 Your project has been created successfully!"),
            CreateLogs::Workspace => f.write_str("🚀 Create a new workspace project successfully!"),
            CreateLogs::Git => f.write_str("🚀 Create a new git project successfully!"),
            CreateLogs::Cargo => f.write_str("🚀 Create a new cargo project successfully!"),
            CreateLogs::GitErr => f.write_str("❌ Create a new git project failed!"),
            CreateLogs::Cancel => f.write_str("❗️ Cancel create project!"),
            CreateLogs::CargoErr => f.write_str("❌ Create a new cargo project failed!"),
        }
    }
}

impl CreateLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

impl Error for CreateLogs {}