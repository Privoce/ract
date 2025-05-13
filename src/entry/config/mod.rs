mod env;
mod chain_env_toml;

pub use env::*;
pub use chain_env_toml::*;

use std::str::FromStr;
use gen_utils::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Configs {
    #[default]
    Env,
    /// config .env file for chain/env.toml
    ChainEnvToml,
}

impl Configs {
    pub fn options() -> Vec<&'static str> {
        vec![".env", "env.toml"]
    }
    
}

impl FromStr for Configs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".env" => Ok(Configs::Env),
            "env.toml" => Ok(Configs::ChainEnvToml),
            _ => Err("Invalid config option!".to_string().into()),
        }
    }
}
