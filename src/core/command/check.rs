use std::{path::PathBuf, process::exit, str::FromStr};

use gen_utils::{
    common::{fs, RustDependence},
    error::Error,
};
use inquire::Select;
use toml_edit::DocumentMut;
use which::which;

use crate::core::{
    entry::{ChainEnvToml, Checks, Tools, Underlayer, UnderlayerTools},
   
    log::{CheckLogs, TerminalLogger},
};

/// Check target toolchain
pub fn run() -> () {
    CheckLogs::Welcome.terminal().rust();

    let check = Select::new("Which you need to check?", Checks::options())
        .prompt()
        .expect("select check failed");

    match Checks::from_str(check).unwrap() {
        Checks::Basic => {
            check_basic();
        }
        Checks::Underlayer => {
            check_underlayer();
        }
        Checks::All => {
            check_basic();
            check_underlayer();
        }
    };

    CheckLogs::Confirm.terminal().success();
}

fn check_basic() -> () {
    let handle = |check: Result<(), Error>| {
        check
            .map_err(|e| {
                TerminalLogger::new(e.to_string().as_str()).error();
                exit(2);
            })
            .unwrap();
    };
    // [rustc] ----------------------------------------------------------------------------------------------
    handle(check_rustc());
    // [cargo] ----------------------------------------------------------------------------------------------
    handle(check_cargo());
    // [git] ------------------------------------------------------------------------------------------------
    handle(check_git());
}

pub fn current_states() -> Result<Tools, Error> {
    // [basic] ----------------------------------------------------------------------------------------------
    let rustc = basic_check("rustc").is_ok();
    let cargo = basic_check("cargo").is_ok();
    let git = basic_check("git").is_ok();
    // [underlayer] -----------------------------------------------------------------------------------------
    let (makepad, gen_ui) = check_makepad()?;
    let makepad = makepad.is_some();
    let gen_ui = gen_ui.is_some();

    Ok(Tools {
        basic: (rustc, cargo, git).into(),
        underlayer: UnderlayerTools::Makepad((makepad, gen_ui).into()),
    })
}

fn check_rustc() -> Result<(), Error> {
    basic_check("rustc").and_then(|_| {
        CheckLogs::Rustc.terminal().success();
        Ok(())
    })
}

fn check_cargo() -> Result<(), Error> {
    basic_check("cargo").and_then(|_| {
        CheckLogs::Cargo.terminal().success();
        Ok(())
    })
}

fn check_git() -> Result<(), Error> {
    basic_check("git").and_then(|_| {
        CheckLogs::Git.terminal().success();
        Ok(())
    })
}

fn basic_check(name: &str) -> Result<(), Error> {
    which(name).map_or_else(|e| Err(e.to_string().into()), |_| Ok(()))
}

fn check_underlayer() -> () {
    let underlayer = Select::new(
        "Which underlayer tool chain you want to check?",
        Underlayer::options(),
    )
    .with_help_message("current support: Makepad")
    .prompt()
    .expect("select underlayer failed");

    match Underlayer::from_str(underlayer).unwrap() {
        Underlayer::Makepad => match check_makepad() {
            Ok((makepad, gen_ui)) => {
                if let Some(makepad) = makepad {
                    CheckLogs::DependenceReady(makepad).terminal().success();
                } else {
                    CheckLogs::DependenceNotFound("makepad-widgets".to_string())
                        .terminal()
                        .warning();
                }
                if let Some(gen_ui) = gen_ui {
                    CheckLogs::DependenceReady(gen_ui).terminal().success();
                } else {
                    CheckLogs::DependenceNotFound("gen_components".to_string())
                        .terminal()
                        .warning();
                }
            }
            Err(e) => {
                TerminalLogger::new(e.to_string().as_str()).error();
                exit(2);
            }
        },
    }
}

fn check_makepad() -> Result<(Option<String>, Option<String>), Error> {
    let dep_exist = |pre: &str,  dep: Option<&str>| -> Option<String> {
        dep.map_or_else(
            || None,
            |dep| {
                return if dep.is_empty() {
                    None
                } else {
                    // check dep path is exist
                    let dep_path = PathBuf::from_str(dep).unwrap();

                    let dep = format!("{} = \"{}\"", pre, dep);
                    RustDependence::from_str(&dep).map_or_else(
                        |_| None,
                        |_| fs::exists_dir(dep_path.as_path()).then_some(dep),
                    )
                };
            },
        )
    };

    // get makepad widget path from chain/env.toml
    // let chain_env_path = real_chain_env_path()?;
    let chain_env_path = ChainEnvToml::path()?;
    fs::read(chain_env_path.as_path()).map_or_else(
        |e| Err(e.to_string().into()),
        |content| {
            let toml_content = content.parse::<DocumentMut>().expect("parse toml failed");
            let makepad_dep = toml_content["dependencies"]["makepad-widgets"].as_str();
            let gen_ui_components_dep = toml_content["dependencies"]["gen_components"].as_str();

            Ok((
                dep_exist("makepad-widgets", makepad_dep),
                dep_exist("gen_components",  gen_ui_components_dep),
            ))
        },
    )
}
