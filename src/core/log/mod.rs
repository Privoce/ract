mod check;
mod create;
mod terminal;
mod init;
mod config;
mod install;
mod level;
mod run;
mod wasm;
mod package;
mod compiler;

pub use run::{RunLogs, StudioLogs, ProjectLogs};
pub use install::InstallLogs;
pub use config::ConfigLogs;
pub use init::InitLogs;
pub use check::CheckLogs;
pub use create::CreateLogs;
pub use terminal::TerminalLogger;
pub use level::LogLevel;
pub use wasm::WasmLogs;
pub use package::PackageLogs;

use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Logs {
    Create(CreateLogs),
    Check(CheckLogs),
    Init(InitLogs),
    Config(ConfigLogs),
    Install(InstallLogs),
    Run(RunLogs),
}

impl Display for Logs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logs::Create(create) => create.fmt(f),
            Logs::Check(check) => check.fmt(f),
            Logs::Init(init) => init.fmt(f),
            Logs::Config(config) => config.fmt(f),
            Logs::Install(install) => install.fmt(f),
            Logs::Run(run) => run.fmt(f),
        }
    }
}

#[allow(dead_code)]
impl Logs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}

impl From<CreateLogs> for Logs {
    fn from(log: CreateLogs) -> Self {
        Logs::Create(log)
    }
}

