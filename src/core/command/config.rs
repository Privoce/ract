use crate::core::{
    entry::{ChainEnvToml, Configs, Env},
    log::{ConfigLogs, TerminalLogger},
};
use gen_utils::{
    common::fs,
    error::{EnvError, Error, FsError},
};
use inquire::{Select, Text};
use std::{path::PathBuf, process::exit, str::FromStr};
use toml_edit::DocumentMut;

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
    let mut env = Env::read()?;
    let content = env.to_string();
    if get_or_set() {
        TerminalLogger::new(&format!(
            "🔸 `{}` is the path for GenUI toolchain env.toml",
            &content
        ))
        .info();
    } else {
        let path = Text::new("Path for the chain env.toml file")
            .with_placeholder("You can write an env.toml file self or use the default path")
            .with_default(&content)
            .prompt()
            .map_err(|e| e.to_string())?;

        let path = PathBuf::from_str(&path).map_err(|e| e.to_string())?;
        if fs::exists(path.as_path()) {
            // write to .env file
            env.set(path);
            return env.write();
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
    let mut chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    if get_or_set() {
        chain_env_toml.dependencies.iter().for_each(|(k, v)| {
            TerminalLogger::new(&format!("🔸 {} = {}", k, fs::path_to_str(v.as_path()))).info();
        });
    } else {
        let options = ChainEnvToml::options();

        let key = Select::new("Which one do you want to config?", options)
            .prompt()
            .map_err(|e| e.to_string())?;

        let default_value = chain_env_toml.dependencies.get(key).map_or_else(
            || {
                Err(Error::Env(EnvError::Get {
                    key: key.to_string(),
                }))
            },
            |v| Ok(v),
        )?;

        let val = Text::new("Path: ")
            .with_placeholder("You should write the path for the dependence repo")
            .with_default(&fs::path_to_str(default_value))
            .prompt()
            .map_err(|e| e.to_string())?;

        let dep_path = PathBuf::from_str(&val).map_err(|e| e.to_string())?;
        // 修改依赖路径
        chain_env_toml
            .dependencies
            .insert(key.to_string(), dep_path.to_path_buf());

        // 如果修改的是makepad-widgets，那么需要更改gen_components中对makepad-widgets的引用
        if key == "makepad-widgets" {
            let _ = chain_env_toml
                .dependencies
                .get("gen_components")
                .map_or_else(
                    || {
                        Err(Error::Env(EnvError::Get {
                            key: "gen_components".to_string(),
                        }))
                    },
                    |path| {
                        let path = path.join("Cargo.toml");
                        let mut cargo_toml = fs::read(path.as_path())?
                            .parse::<DocumentMut>()
                            .map_err(|e| e.to_string())?;
                        // change makepad-widgets from dependencies == dep_path
                        cargo_toml["dependencies"]["makepad-widgets"] =
                            toml_edit::value(fs::path_to_str(&dep_path));
                        fs::write(path.as_path(), &cargo_toml.to_string())
                    },
                )?;
        }

        return chain_env_toml.write();
    }

    Ok(())
}
