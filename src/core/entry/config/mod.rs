mod env;
mod chain_env_toml;

use std::str::FromStr;

use gen_utils::error::Error;

pub enum Configs {
    Env,
    /// config .env file for chain/env.toml
    ChainEnvToml,
}

impl Configs {
    pub fn options() -> Vec<&'static str> {
        vec!["env", "chain_env_toml"]
    }
}

impl FromStr for Configs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "env" => Ok(Configs::Env),
            "chain_env_toml" => Ok(Configs::ChainEnvToml),
            _ => Err("Invalid config option!".to_string().into()),
        }
    }
}
