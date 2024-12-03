use std::{fmt::Display, str::FromStr};

use gen_utils::error::{ConvertError, Error};

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
        match self {
            LogLevel::Info => f.write_str("info"),
            LogLevel::Debug => f.write_str("debug"),
            LogLevel::Error => f.write_str("error"),
            LogLevel::Warn => f.write_str("warn"),
            LogLevel::Trace => f.write_str("trace"),
            LogLevel::Off => f.write_str("off"),
        }
    }
}