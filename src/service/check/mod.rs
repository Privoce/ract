mod item;
pub use item::*;
use std::path::PathBuf;

use gen_utils::error::Error;

use which::which;

use crate::{
    common::is_empty_dir,
    entry::{ChainEnvToml, MakepadState, ToolState, Underlayer},
};

/// ## Check basic toolchain
/// 1. rustc
/// 2. cargo
/// 3. git
pub fn check_basic() -> Vec<CheckItem> {
    vec![check_rustc(), check_cargo(), check_git()]
}

pub fn current_states() -> Result<ToolState, Error> {
    // [basic] ----------------------------------------------------------------------------------------------
    let rustc = check_rustc().state;
    let cargo = check_cargo().state;
    let git = check_git().state;
    // [underlayer] -----------------------------------------------------------------------------------------
    let ((makepad, _), (gen_ui, _)) = makepad_exist()?;

    Ok(ToolState {
        basic: (rustc, cargo, git).into(),
        underlayer: MakepadState::new(makepad, gen_ui),
    })
}

pub fn check_rustc() -> CheckItem {
    basic_check("rustc".to_string())
}

pub fn check_cargo() -> CheckItem {
    basic_check("cargo".to_string())
}

pub fn check_git() -> CheckItem {
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
            !is_empty_dir(makepad_widgets_path)?,
            makepad_widgets_path.cloned(),
        ),
        (
            !is_empty_dir(gen_components_path)?,
            gen_components_path.cloned(),
        ),
    ))
}
