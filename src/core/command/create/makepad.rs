use std::{path::Path, process::Command};

use gen_utils::{common::fs, error::Error};

use crate::core::{
    constant::{MAKEPAD_APP_RS, MAKEPAD_LIB_RS, MAKEPAD_MAIN_RS},
    entry::ProjectInfo,
    log::{CreateLogs, TerminalLogger},
};

use super::git_init;

pub fn create<P>(path: P, info: ProjectInfo, git: bool) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    TerminalLogger::new(&format!(
        "🛠️ ract is creating a new Makepad project `{}` in: {}",
        info.name,
        path.as_ref().display()
    ))
    .info();
    // create a default makepad project
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
                    // [init git repository] ----------------------------------------------------------------------
                    if git {
                        git_init(path.as_path())?;
                    }
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
