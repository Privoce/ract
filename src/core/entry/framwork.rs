use std::{fmt::Display, str::FromStr};

use gen_utils::error::Error;
use toml_edit::{value, Item};

#[derive(Debug, Clone, Copy, Default)]
pub enum FrameworkType {
    #[default]
    GenUI,
    Makepad,
}

impl Display for FrameworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkType::GenUI => f.write_str("gen_ui"),
            FrameworkType::Makepad => f.write_str("makepad"),
        }
    }
}

impl FrameworkType {
    pub fn options() -> Vec<&'static str> {
        vec!["gen_ui", "makepad"]
    }
}

impl From<FrameworkType> for Item {
    fn from(f: FrameworkType) -> Self {
        value(f.to_string())
    }
}

impl FromStr for FrameworkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gen_ui" => Ok(FrameworkType::GenUI),
            "makepad" => Ok(FrameworkType::Makepad),
            _ => Err(Error::from("FrameworkType not found")),
        }
    }
}
