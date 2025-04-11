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
pub mod error;

use std::borrow::Cow;

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


use super::entry::Language;

pub trait LogExt {
    /// ## Convert the log to a terminal logger
    /// TerminalLogger is a logger that can be used to log to the terminal and link with i18n
    fn terminal(&self, lang: &Language) -> TerminalLogger;
}

// impl LogExt for gen_utils::error::Error {
//     fn terminal(&self, lang: &Language) -> TerminalLogger {
//         match self {
//             gen_utils::error::Error::Parse(parse_error) => parse_error.terminal(lang),
//             gen_utils::error::Error::Convert(convert_error) => todo!(),
//             gen_utils::error::Error::FromDynError(_) => todo!(),
//             gen_utils::error::Error::Env(env_error) => todo!(),
//             gen_utils::error::Error::Compiler(compiler_error) => todo!(),
//             gen_utils::error::Error::Fs(fs_error) => todo!(),
//         }
//     }
// }

// impl LogExt for gen_utils::error::ParseError {
//     fn terminal(&self, lang: &Language) -> TerminalLogger {
//         let gen_utils::error::ParseError { target, other, ty } = self;

//         let ty = match ty {
//             gen_utils::error::ParseType::RustDep => "Rust Dependency",
//             gen_utils::error::ParseType::Template => "GenUI Template",
//             gen_utils::error::ParseType::DSLBind => "GenUI DSL Bind",
//             gen_utils::error::ParseType::Toml => "TOML",
//             gen_utils::error::ParseType::Color(_) => "GenUI Color",
//             gen_utils::error::ParseType::Conf => "Configuration",
//             gen_utils::error::ParseType::Other(other) => other,
//         };

//         TerminalLogger {
//             output: t!(
//                 "error.parse",
//                 locale = lang.as_str(),
//                 ty = ty,
//                 target = target,
//                 reason = other.as_ref().unwrap_or(&" - ".to_string())
//             ),
//         }
//     }
// }
