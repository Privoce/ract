use std::{fmt::Display, str::FromStr};

use gen_utils::error::{ConvertError, Error};
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