use std::path::{Path, PathBuf};

use gen_utils::{common::fs, error::Error};

use crate::core::{
    entry::{ProjectInfo, RactToml, WorkspaceInfo},
    log::TerminalLogger, util::create_workspace,
};

use super::ProjectInfoType;

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


    Ok(path)
}



// create GenUI project depend on project info
fn create_project<P>(
    path: P,
    info: &ProjectInfo,
   
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let workspace_path = path.as_ref().join(&info.name);
    // [LICENSE] -----------------------------------------------------------------------------------------
    info.write_license(workspace_path.as_path())?;
    // [use cargo to create a new project] ---------------------------------------------------------------
    // std::process::Command::new("cargo")
    //     .arg("new")
    //     .arg("--bin")
    //     .arg(&info.name)
    //     .current_dir(workspace_path.as_path())
    //     .output()
    //     .map_or_else(
    //         |e| {
    //             TerminalLogger::new(e.to_string().as_str()).error();
    //             exit(2);
    //         },
    //         |out| {
    //             if out.status.success() {
    //                 CreateLogs::Cargo.terminal().success();
    //                 // [create dir: resources, views, components] ------------------------------------------
    //                 let ui_dir_path = workspace_path.join(&info.name);
    //                 for path in ["resources", "views", "components"].iter() {
    //                     let _ = fs::create_dir(ui_dir_path.join(path).as_path())?;
    //                 }
    //                 // [handle Cargo.toml] -----------------------------------------------------------------
    //                 let _ = info.write_gen_ui_cargo_toml(ui_dir_path.as_path())?;
    //                 // [create config files: gen_ui.toml, .gen_ui_cache] -----------------------------------
    //                 let _ = fs::create_new(ui_dir_path.join(".gen_ui_cache").as_path())?;
    //                 // - [gen_ui.toml] ---------------------------------------------------------------------
    //                 underlayer.write_gen_ui_toml(ui_dir_path.as_path())?;
    //                 // TODO! [create files in resources, views, components] --------------------------------
    //                 CreateLogs::Ui.terminal().success();
    //                 Ok(())
    //             } else {
    //                 Err(CreateLogs::CargoErr.to_string().into())
    //             }
    //         },
    //     )
    Ok(())
}
