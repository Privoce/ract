mod add;
mod check;
pub mod compiler;
mod config;
mod create;
mod init;
mod install;
mod level;
mod package;
mod run;
mod terminal;
mod wasm;
mod other;

pub use add::AddLogs;
pub use check::CheckLogs;
use compiler::CompilerLogs;
pub use config::ConfigLogs;
pub use create::CreateLogs;
pub use init::InitLogs;
pub use install::InstallLogs;
pub use level::LogLevel;
pub use package::PackageLogs;
pub use run::{ProjectLogs, RunLogs, StudioLogs};
use rust_i18n::t;
pub use terminal::TerminalLogger;
pub use wasm::WasmLogs;

use std::{error::Error, fmt::Display};

use super::entry::Language;
