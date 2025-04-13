use std::{fmt::Display, str::FromStr};

use colored::{ColoredString, Colorize};
use gen_utils::error::{ConvertError, Error};
use log::{Level, LevelFilter};
use ratatui::style::Color;
use toml_edit::{Formatted, Value};

#[derive(Debug, Clone, Copy, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Error,
    Warn,
    Trace,
    Off,
}

impl LogLevel {
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Info => Color::Blue,
            LogLevel::Debug => Color::Cyan,
            LogLevel::Error => Color::Red,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Trace => Color::White,
            LogLevel::Off => Color::White,
        }
    }
    pub fn fmt_level(&self) -> &str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Trace => "TRACE",
            LogLevel::Off => "OFF",
        }
    }
}

impl FromStr for LogLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "trace" => Ok(Self::Trace),
            "off" => Ok(Self::Off),
            _ => Err(ConvertError::FromTo {
                from: "str".to_string(),
                to: format!("LogLevel, Invalid: {}", s),
            }
            .into()),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&LogLevel> for Value {
    fn from(value: &LogLevel) -> Self {
        Value::String(Formatted::new(
            match value {
                LogLevel::Info => "info",
                LogLevel::Debug => "debug",
                LogLevel::Error => "error",
                LogLevel::Warn => "warn",
                LogLevel::Trace => "trace",
                LogLevel::Off => "off",
            }
            .to_string(),
        ))
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Off => LevelFilter::Off,
        }
    }
}

pub struct LevelColord(Level);

impl LevelColord {
    pub fn colored(&self) -> ColoredString {
        match self.0 {
            Level::Info => "INFO".bright_blue(),
            Level::Debug => "DEBUG".cyan(),
            Level::Error => "ERROR".bright_red(),
            Level::Warn => "WARN".bright_yellow(),
            Level::Trace => "TRACE".purple(),
        }
    }
}

impl From<Level> for LevelColord {
    fn from(value: Level) -> Self {
        Self(value)
    }
}
