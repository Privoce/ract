use std::path::{Path, PathBuf};

use gen_utils::{
    common::{fs, ToToml},
    error::Error,
};

use crate::core::{
    entry::{ProjectInfo, RactToml, WorkspaceInfo},
    log::{CreateLogs, TerminalLogger},
    util::create_workspace,
};

pub fn create<P>(path: P, info: &WorkspaceInfo, ract_toml: &RactToml) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
{
    TerminalLogger::new(&format!(
        "🛠️ ract is creating a new GenUI workspace `{}` in: {}",
        &info.name,
        fs::path_to_str(path.as_ref())
    ))
    .info();
    // [rust workspace path] -------------------------------------------------------
    let path = path.as_ref().join(&info.name);
    // [workspace Cargo.toml] ------------------------------------------------------
    let cargo_toml = info.workspace_members_toml().to_string();
    // [create a new wrokspace] ----------------------------------------------------
    let _ = create_workspace(path.as_path(), &cargo_toml, ract_toml)?;
    // [create projects] -----------------------------------------------------------
    for project in &info.members {
        let _ = create_project(path.as_path(), project)?;
    }

    Ok(path)
}

// create GenUI project depend on project info
fn create_project<P>(path: P, info: &ProjectInfo) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // [LICENSE] -----------------------------------------------------------------------------------------
    info.write_license(path.as_ref())?;
    // [use cargo to create a new project] ---------------------------------------------------------------
    std::process::Command::new("cargo")
        .args(&["new", "--bin", &info.name, "--vcs", "none"])
        .current_dir(path.as_ref())
        .output()
        .map_or_else(
            |e| Err(Error::from(e.to_string())),
            |out| {
                if out.status.success() {
                    CreateLogs::Cargo.terminal().success();
                    // [create dir: resources, views, components] ------------------------------------------
                    let ui_dir_path = path.as_ref().join(&info.name);
                    for path in ["resources", "views", "components"].iter() {
                        let _ = fs::create_dir(ui_dir_path.join(path))?;
                    }
                    // [handle Cargo.toml] -----------------------------------------------------------------
                    let _ = info.write(ui_dir_path.join("Cargo.toml"))?;
                    // [create config files: gen_ui.toml, .gen_ui_cache] -----------------------------------
                    let _ = fs::create_new(ui_dir_path.join(".gen_ui_cache"))?;
                    // - [gen_ui.toml] ---------------------------------------------------------------------
                    info.write_gen_ui_toml(ui_dir_path.as_path())?;
                    Ok(())
                } else {
                    Err(CreateLogs::CargoErr.to_string().into())
                }
            },
        )
}