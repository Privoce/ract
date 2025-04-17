mod item;
pub use item::*;
use std::{path::PathBuf, process::exit, str::FromStr};

use gen_utils::{
    common::{fs, RustDependence},
    error::Error,
};
use inquire::Select;
use rust_i18n::t;
use toml_edit::DocumentMut;
use which::which;

use crate::{
    entry::{ChainEnvToml, Checks, Language, Tools, Underlayer, UnderlayerTools},
    log::{CheckLogs, LogExt, LogItem, TerminalLogger},
};

/// Check target toolchain
pub fn run() -> () {
    let lang = Language::from_conf();

    let check = Select::new(&CheckLogs::Select.t(&lang).to_string(), Checks::options())
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

    // CheckLogs::Confirm.terminal().success();
    LogItem::success(CheckLogs::Complete.t(&lang).to_string()).log();
}

/// ## Check basic toolchain
/// 1. rustc
/// 2. cargo
/// 3. git
pub fn check_basic() -> Vec<CheckItem> {
    vec![check_rustc(), check_cargo(), check_git()]
}

pub fn current_states() -> Result<Tools, Error> {
    // [basic] ----------------------------------------------------------------------------------------------
    // let rustc = basic_check("rustc").is_ok();
    // let cargo = basic_check("cargo").is_ok();
    // let git = basic_check("git").is_ok();
    // [underlayer] -----------------------------------------------------------------------------------------
    let (makepad, gen_ui) = check_makepad()?;
    let makepad = makepad.is_some();
    let gen_ui = gen_ui.is_some();

    // Ok(Tools {
    //     basic: (rustc, cargo, git).into(),
    //     underlayer: UnderlayerTools::Makepad((makepad, gen_ui).into()),
    // })
    Ok(Tools {
        basic: Default::default(),
        underlayer: UnderlayerTools::Makepad((makepad, gen_ui).into()),
    })
}

fn check_rustc() -> CheckItem {
    basic_check("rustc".to_string())
}

fn check_cargo() -> CheckItem {
    basic_check("cargo".to_string())
}

fn check_git() -> CheckItem {
    basic_check("git".to_string())
}

fn basic_check(name: String) -> CheckItem {
    let mut item: CheckItem = which(&name).into();
    item.name = name;
    item
}

pub fn check_underlayer() -> () {
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

pub fn check_makepad() -> Result<(Option<String>, Option<String>), Error> {
    let dep_exist = |pre: &str, dep: Option<&str>| -> Option<String> {
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
                dep_exist("gen_components", gen_ui_components_dep),
            ))
        },
    )
}
