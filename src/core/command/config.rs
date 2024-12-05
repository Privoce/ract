use std::{path::PathBuf, process::exit, str::FromStr};

use gen_utils::{
    common::fs,
    error::{Error, FsError},
};
use inquire::{Select, Text};
use toml_edit::{value, DocumentMut};

use crate::core::{
    entry::Configs,
    env::{env_path, match_chain_value, real_chain_env_path},
    log::{ConfigLogs, TerminalLogger},
};

pub fn run() -> () {
    ConfigLogs::Welcome.terminal().rust();
    ConfigLogs::Desc.terminal().info();

    let config = Select::new("Which env file do you want to config?", Configs::options())
        .prompt()
        .unwrap();

    match Configs::from_str(config).unwrap() {
        Configs::Env => {
            if let Err(e) = config_env() {
                ConfigLogs::EnvFail.terminal().error();
                TerminalLogger::new(&e.to_string()).error();
                exit(2);
            }
        }
        Configs::ChainEnvToml => {
            if let Err(e) = config_env_toml() {
                ConfigLogs::EnvFail.terminal().error();
                TerminalLogger::new(&e.to_string()).error();
                exit(2);
            }
        }
    }

    ConfigLogs::Confirm.terminal().success();
}

fn config_env() -> Result<(), Error> {
    let env_path = env_path();

    if get_or_set() {
        let content = fs::read(env_path.as_path())?;
        TerminalLogger::new(&format!(
            "🔸 `{}` is the path for GenUI toolchain env.toml",
            content
        ))
        .info();
    } else {
        let path = Text::new("Path for the chain env.toml file")
            .with_placeholder("You can write an env.toml file self or use the default path")
            .with_default(env_path.as_path().to_str().unwrap())
            .prompt()
            .map_err(|e| e.to_string())?;

        let path = PathBuf::from_str(&path).unwrap();
        if fs::exists(path.as_path()) {
            // write to .env file
            fs::write(env_path.as_path(), path.as_path().to_str().unwrap())?;
            return Ok(());
        } else {
            return Err(FsError::FileNotFound(path).into());
        }
    }
    Ok(())
}

fn get_or_set() -> bool {
    let is_get = Select::new("Get or Set Config?", vec!["get", "set"])
        .prompt()
        .unwrap();

    is_get == "get"
}

fn config_env_toml() -> Result<(), Error> {
    let chain_env_toml = real_chain_env_path()?;
    // read the chain env.toml file and convert to Document then get ["dependencies"]
    let mut content = fs::read(chain_env_toml.as_path())?
        .parse::<DocumentMut>()
        .map_err(|e| e.to_string())?;
    let dependencies = &content["dependencies"].as_table();

    if get_or_set() {
        if let Some(dependencies) = dependencies {
            for (key, value) in dependencies.iter() {
                TerminalLogger::new(&format!("🔸 {} = {}", key, value)).info();
            }
        } else {
            return Err("dependencies not found, please check the env.toml and ensure the format! Or you can run `ract init` then run `ract install` and select `default` to rebuild".to_string().into());
        }
    } else {
        if let Some(dependencies) = dependencies {
            let options = dependencies
                .iter()
                .map(|(key, _)| key.to_string())
                .collect::<Vec<String>>();

            let key = Select::new("Which one do you want to config?", options)
                .prompt()
                .map_err(|e| e.to_string())?;

            let default_value = match_chain_value(&key);
            let val = Text::new("Path: ")
            .with_placeholder("You should write the path for the dependence repo")
            .with_default(&default_value)
            .prompt()
            .map_err(|e| e.to_string())?;

            content["dependencies"][key] = value(fs::path_to_str(val));

            // write back
            fs::write(chain_env_toml.as_path(), content.to_string().as_str())?;
        } else {
            return Err("dependencies not found, please check the env.toml and ensure the format! Or you can run `ract init` then run `ract install` and select `default` to rebuild".to_string().into());
        }
    }

    Ok(())
}
