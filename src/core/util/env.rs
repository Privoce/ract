use std::{env::current_exe, path::PathBuf, str::FromStr};

use gen_utils::{
    common::fs,
    error::{EnvError, Error, ParseError, ParseType},
};
use toml_edit::DocumentMut;

/// get `/chain/env.toml` path
pub fn chain_env_path() -> PathBuf {
    exe_path().join("chain").join("env.toml")
}

/// get `/chain` path
pub fn chain_path() -> PathBuf {
    exe_path().join("chain")
}

/// get `.env` file path
pub fn env_path() -> PathBuf {
    exe_path().join(".env")
}

pub fn exe_path() -> PathBuf {
    let mut path = current_exe().expect("get current exe path failed");
    path.pop();
    path
}

/// get real chain env path depend on `.env` file
pub fn real_chain_env_path() -> Result<PathBuf, Error> {
    let env_path = env_path();

    if let Ok(content) = fs::read(env_path.as_path()) {
        return Ok(PathBuf::from_str(&content).map_err(|e| e.to_string())?);
    }

    return Err(EnvError::Get {
        key: ".env file".to_string(),
    }
    .into());
}

pub fn makepad_widgets_path() -> PathBuf {
    chain_path().join("makepad")
}

pub fn gen_components_path() -> PathBuf {
    chain_path().join("gen_components")
}

pub fn match_chain_value(key: &str) -> String {
    let handle = |p: PathBuf| -> String { fs::path_to_str(p.as_path()) };

    match key {
        "makepad-widgets" => handle(makepad_widgets_path()),
        "gen_components" => handle(gen_components_path()),
        _ => panic!("Invalid key!"),
    }
}

pub fn real_chain_env_toml() -> Result<DocumentMut, Error> {
    let chain_env_toml = real_chain_env_path()?;
    let content = fs::read(chain_env_toml.as_path())?;
    content
        .parse::<DocumentMut>()
        .map_err(|e| Error::Parse(ParseError::new(e.to_string().as_str(), ParseType::Toml)))
}
