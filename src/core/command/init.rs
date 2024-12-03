use std::{path::PathBuf, str::FromStr};

use gen_utils::{
    common::fs,
    error::{EnvError, Error},
};

use crate::core::{
    constant::DEFAULT_ENV_TOML,
    env::{chain_env_path, chain_path, env_path, gen_components_path, makepad_widgets_path},
    log::InitLogs,
};

pub fn check() {
    // get .env file and read content
    if let Ok(_) = check_env_file() {
        return;
    } else {
        InitLogs::Init.terminal().info();
        create_env_file();
        create_chain();
        InitLogs::Confirm.terminal().success();
    }
}

pub fn run() {
    InitLogs::Init.terminal().info();
    create_env_file();
    create_chain();
    InitLogs::Confirm.terminal().success();
}

/// check if .env file exists and read content then back chain path
fn check_env_file() -> Result<PathBuf, Error> {
    let env_path = env_path();
    if let Ok(content) = fs::read(env_path.as_path()) {
        if content.trim().is_empty() {
            return Err(EnvError::Empty.into());
        } else {
            // convert to path and check if exists
            PathBuf::from_str(&content).map_or_else(
                |e| Err(e.to_string().into()),
                |env_path| {
                    if env_path.exists() {
                        Ok(env_path)
                    } else {
                        Err(EnvError::Get {
                            key: ".env file".to_string(),
                        }
                        .into())
                    }
                },
            )
        }
    } else {
        return Err(EnvError::Get {
            key: ".env file".to_string(),
        }
        .into());
    }
}

fn create_env_file() {
    let env_path = env_path();
    let chain_env = chain_env_path();
    fs::write(env_path.as_path(), chain_env.as_path().to_str().unwrap())
        .expect("write env.toml failed");
}

fn create_chain() {
    let chain_path = chain_path();
    let env_toml_path = chain_env_path();
    let makepad_path = makepad_widgets_path();
    let gen_component_path = gen_components_path();
    // create a new file: ${current_exe}/chain/env.toml with default content
    fs::create_dir(chain_path.as_path()).expect("create chain dir failed");
    let content = DEFAULT_ENV_TOML
        .replace(
            "${makepad-widgets}",
            &fs::path_to_str(makepad_path.as_path()),
        )
        .replace(
            "${gen_components}",
            &fs::path_to_str(gen_component_path.as_path()),
        );
    fs::write(env_toml_path.as_path(), &content).expect("write env.toml failed");
    InitLogs::Chain.terminal().success();
}
