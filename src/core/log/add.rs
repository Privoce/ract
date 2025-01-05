use std::fmt::Display;

use super::terminal::TerminalLogger;

#[derive(Debug, Clone)]
pub enum AddLogs {
    DownloadFailed(String),
    DownloadSuccess(String),
    Downloading(String),
}

impl Display for AddLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddLogs::DownloadFailed(reason) => write!(f, "❌ Download plugin failed: {}", reason),
            AddLogs::DownloadSuccess(name) => write!(f, "🎉 Download plugin: {} success", name),
            AddLogs::Downloading(name) => {
                write!(f, "🔸 Downloading plugin: {} . Please wait...", name)
            }
        }
    }
}

impl AddLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}
