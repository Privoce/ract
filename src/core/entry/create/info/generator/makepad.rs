use std::{path::Path, process::Command};

use gen_utils::{common::fs, error::Error};

use crate::core::{
    constant::{MAKEPAD_APP_RS, MAKEPAD_LIB_RS, MAKEPAD_MAIN_RS},
    entry::{ProjectInfo, RactToml, WorkspaceInfo},
    log::{CreateLogs, TerminalLogger},
    util,
};

pub fn create_workspace<P>(path: P, info: &WorkspaceInfo, ract_toml: &RactToml) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    TerminalLogger::new(&format!(
        "🛠️ ract is creating a new Makepad workspace `{}` in: {}",
        info.name,
        fs::path_to_str(path.as_ref())
    ))
    .info();
    // [rust workspace path] -------------------------------------------------------
    let path = path.as_ref().join(info.name.as_str());
    // [workspace Cargo.toml] ------------------------------------------------------
    let cargo_toml = info.workspace_members_toml().to_string();
    let _ = util::create_workspace(path.as_path(), &cargo_toml, ract_toml)?;

    Ok(())
}

/// create a default makepad project
pub fn crate_project<P>(path: P, info: &ProjectInfo) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    TerminalLogger::new(&format!(
        "🛠️ ract is creating a new Makepad project `{}` in: {}",
        info.name,
        fs::path_to_str(path.as_ref())
    ))
    .info();
    // [use cargo new --bin to create] --------------------------------------------------------------------
    Command::new("cargo")
        .args(&["new", "--bin", info.name.as_str()])
        .current_dir(path.as_ref())
        .output()
        .map_or_else(
            |e| {
                TerminalLogger::new(e.to_string().as_str()).error();
                Err(e.to_string().into())
            },
            |out| {
                if out.status.success() {
                    CreateLogs::Cargo.terminal().success();
                    let path = path.as_ref().join(info.name.as_str());
                    // [handle the Cargo.toml] --------------------------------------------------------------------
                    info.write_makepad_cargo_toml(path.as_path())?;
                    // [write a signature file] -------------------------------------------------------------------
                    fs::write(path.join(".ract"), "makepad")?;
                    // [create default files: lib.rs, app.rs] -----------------------------------------------------
                    create_lib_rs(path.as_path())?;
                    create_main_rs(path.as_path(), &info.name)?;
                    create_app_rs(path.as_path())?;
                    // [create a resources folder] ----------------------------------------------------------------
                    let _ = fs::create_dir(path.join("resources"))?;
                    // [LICENSE] ----------------------------------------------------------------------------------
                    let _ = info.write_license(path.as_path());
                    // finish
                    Ok(())
                } else {
                    Err(Error::from("Makepad project created failed!"))
                }
            },
        )
}

fn create_lib_rs<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    fs::write(path.as_ref().join("src").join("lib.rs"), MAKEPAD_LIB_RS)
}

fn create_main_rs<P>(path: P, name: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    fs::write(
        path.as_ref().join("src").join("main.rs"),
        &MAKEPAD_MAIN_RS.replace("${project_name}", name),
    )
}

fn create_app_rs<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    fs::write(path.as_ref().join("src").join("app.rs"), MAKEPAD_APP_RS)
}
