use std::{fmt::Display, str::FromStr};

use gen_utils::error::Error;

use super::Underlayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Checks {
    #[default]
    Basic,
    Underlayer(Underlayer),
    All(Underlayer),
}

impl FromStr for Checks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" | "Basic" => Ok(Checks::Basic),
            "underlayer" | "Underlayer" => Ok(Checks::Underlayer(Underlayer::default())),
            "all" | "All" => Ok(Checks::All(Underlayer::default())),
            _ => Err(format!("unknown check: {}", s).into()),
        }
    }
}

impl Display for Checks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checks::Basic => f.write_str("Basic"),
            Checks::Underlayer(_) => f.write_str("Underlayer"),
            Checks::All(_) => f.write_str("All"),
        }
    }
}

impl Checks {
    pub fn options() -> Vec<&'static str> {
        vec!["Basic", "Underlayer", "All"]
    }
}
