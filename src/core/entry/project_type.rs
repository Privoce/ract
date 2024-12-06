use std::{fmt::Display, str::FromStr};

use gen_utils::error::Error;

#[derive(Debug, Clone, Copy, Default)]
pub enum ProjectType {
    #[default]
    GenUI,
    Makepad,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::GenUI => f.write_str("gen_ui"),
            ProjectType::Makepad => f.write_str("makepad"),
        }
    }
}

impl ProjectType {
    pub fn options() -> Vec<&'static str> {
        vec!["gen_ui", "makepad"]
    }
}

impl FromStr for ProjectType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gen_ui" => Ok(ProjectType::GenUI),
            "makepad" => Ok(ProjectType::Makepad),
            _ => Err(Error::from("ProjectType not found")),
        }
    }
}
