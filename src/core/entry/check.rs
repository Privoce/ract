use std::{fmt::Display, str::FromStr};

use gen_utils::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Checks {
    Basic,
    Underlayer,
    #[default]
    All,
}

impl FromStr for Checks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" | "Basic" => Ok(Checks::Basic),
            "underlayer" | "Underlayer" => Ok(Checks::Underlayer),
            "all" | "All" => Ok(Checks::All),
            _ => Err(format!("unknown check: {}", s).into()),
        }
    }
}

impl Display for Checks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checks::Basic => f.write_str("Basic"),
            Checks::Underlayer => f.write_str("Underlayer"),
            Checks::All => f.write_str("All"),
        }
    }
}

impl Checks {
    pub fn options() -> Vec<&'static str> {
        vec!["Basic", "Underlayer", "All"]
    }
}
