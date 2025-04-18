mod item;
pub use item::*;
use std::{
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

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
        Checks::Underlayer(_) => {
            // check_underlayer();
        }
        Checks::All(_) => {
            check_basic();
            // check_underlayer();
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
    let ((makepad, _), (gen_ui, _)) = makepad_exist()?;

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

/// ## Check underlayer toolchain
/// 1. makepad (current support)
pub fn check_underlayer(underlayer: Underlayer) -> Result<Vec<CheckItem>, Error> {
    match underlayer {
        Underlayer::Makepad => check_makepad(),
    }
}

pub fn check_makepad() -> Result<Vec<CheckItem>, Error> {
    let ((makepad_exist, makepad_widgets_path), (gen_components_exist, gen_components_path)) =
        makepad_exist()?;

    Ok(vec![
        CheckItem::new(
            "makepad_widgets".to_string(),
            makepad_widgets_path,
            makepad_exist,
        ),
        CheckItem::new(
            "gen_components".to_string(),
            gen_components_path,
            gen_components_exist,
        ),
    ])
}

fn makepad_exist() -> Result<((bool, Option<PathBuf>), (bool, Option<PathBuf>)), Error> {
    let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let makepad_widgets_path = chain_env_toml.makepad_widgets_path();
    let gen_components_path = chain_env_toml.gen_components_path();

    Ok((
        (
            is_empty_dir(makepad_widgets_path)?,
            makepad_widgets_path.cloned(),
        ),
        (
            is_empty_dir(gen_components_path)?,
            gen_components_path.cloned(),
        ),
    ))
}

fn is_empty_dir<P>(path: Option<P>) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    if let Some(path) = path {
        if fs::exists_dir(path.as_ref()) {
            let mut entries = std::fs::read_dir(path).map_err(|e| Error::from(e.to_string()))?;
            return Ok(entries.next().is_none());
        }
    }

    Ok(false)
}
