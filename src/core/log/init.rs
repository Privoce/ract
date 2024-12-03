use std::fmt::Display;

use super::TerminalLogger;

#[derive(Debug)]
pub enum InitLogs {
    Init,
    Confirm,
    Chain
}

impl Display for InitLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitLogs::Init => f.write_str("🚀 Start to init ract..."),
            InitLogs::Confirm => f.write_str("🎉 Init ract successfully!"),
            InitLogs::Chain => f.write_str("✅ Chain init successfully!"),
        }
    }
}

impl InitLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}
