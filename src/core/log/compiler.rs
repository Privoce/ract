//! # GenUI Logger
//!
//! ## Logo
//!
//! You can control whether the logo is printed using the system environment variable `GENUI_LOGO` or through the configuration file in TOML format.
//!
//! - For more details, see [GenUI Environment Setup](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/env.html).
//! - For configuration, see [GenUI Config TOML](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/conf.html).
//!
//! Example:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>>
//!
//!      _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/
//!   _/        _/        _/_/    _/  _/    _/    _/
//!  _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/
//! _/    _/  _/        _/    _/_/  _/    _/    _/
//!  _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/
//!
//! ```
//!
//! ## Services
//!
//! The GenUI Logger provides detailed information about the state of various services. Here are some log examples:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Source Generator Service started successfully!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 App is running...
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Watcher Service started successfully!
//! ```
//!
//! ## Compile Timing
//!
//! The logger also tracks and displays compile timings, helping you monitor the compilation process:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
//! ```

use std::fmt::Display;

use crate::core::{constant::LOGO, log::level::LevelColord};
use colored::Colorize;
use env_logger::{Builder, Env, WriteStyle};
use gen_utils::common::time::local_time_default;
use log::{error, info, warn};
use std::io::Write;

use super::{LogLevel, TerminalLogger};

/// # Init Log
/// init GenUI log service. It will read the system environment variable `GENUI_LOGO` and `GENUI_LOG_LEVEL` to set the log level and print the logo.
/// If the system environment variable is not set, it will read the configuration file in the project root path.
/// If the configuration file is not found, it will use the default value.
/// > This function should be called before any other service is started.
pub fn init(log_level: LogLevel) -> () {
    // [init log env] -----------------------------------------------------------------------------------------
    // let env = Env::default()
    //     .filter_or("GENUI_LOG_LEVEL", log_level.to_string())
    //     .write_style_or("GENUI_LOG_STYLE", "always");
    // [build log] -----------------------------------------------------------------------------------------
    let mut builder = Builder::new();

    builder
        .filter_level(log_level.into())
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            let title = "GenUI-Compiler".truecolor(255, 112, 67);
            let timestamp = local_time_default().bright_blue();
            let level = LevelColord::from(record.level()).colored();
            writeln!(
                buf,
                "{} :: [{}] :: {} >>> {}",
                title,
                timestamp,
                level,
                record.args()
            )
        })
        .init();

    CompilerLogs::Init.compiler().info();
}

pub struct CompilerLogger {
    pub output: String,
}

impl CompilerLogger {
    pub fn new(s: &str) -> CompilerLogger {
        CompilerLogger {
            output: s.to_string(),
        }
    }
    pub fn info(&self) -> () {
        info!("{}", self.output.white());
    }

    pub fn warn(&self) -> () {
        warn!("{}", self.output.bright_yellow());
    }

    pub fn error(&self) -> () {
        error!("{}", self.output.bright_red());
    }

    pub fn error_and_exit(&self) -> ! {
        error!("{}", self.output.bright_red());
        std::process::exit(1)
    }
}

impl From<String> for CompilerLogger {
    fn from(value: String) -> Self {
        CompilerLogger { output: value }
    }
}

impl From<CompilerLogs> for CompilerLogger {
    fn from(value: CompilerLogs) -> Self {
        CompilerLogger {
            output: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompilerLogs {
    Init,
    Logo,
}

impl Display for CompilerLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerLogs::Init => f.write_str("🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!"),
            CompilerLogs::Logo => f.write_str(LOGO),
        }
    }
}

impl CompilerLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
    pub fn compiler(&self) -> CompilerLogger {
        (*self).into()
    }
}
