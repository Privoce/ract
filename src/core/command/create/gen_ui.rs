use gen_utils::{common::fs, error::Error};
use std::{path::Path, process::exit};

use crate::core::{
    constant::DEFAULT_CARGO_TOML_CONTENT,
    entry::{CompileTarget, ProjectInfo},
    log::{CreateLogs, TerminalLogger},
};

use super::git_init;

pub fn create<P>(
    path: P,
    info: ProjectInfo,
    underlayer: CompileTarget,
    git: bool,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    match path.as_ref().canonicalize() {
        Ok(path) => {
            TerminalLogger::new(&format!(
                "🛠️ ract is creating a new GenUI project `{}` in: {}",
                info.name,
                path.display()
            ))
            .info();
            // create a rust workspace project
            let _ = create_rust_workspace(path.as_path(), &info.name, git)?;
            // create a GenUI project in the workspace
            let _ = create_gen_ui_project(path.as_path(), &info, underlayer)?;
            Ok(())
        }
        Err(e) => Err(e.to_string().into()),
    }
}

fn create_rust_workspace<P>(path: P, name: &str, git: bool) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // use name as workspace project name
    let workspace_path = path.as_ref().join(name);
    // create workspace project
    match std::fs::create_dir(workspace_path.as_path()) {
        Ok(_) => {
            // create Cargo.toml
            let cargo_toml = workspace_path.join("Cargo.toml");
            // write default Cargo.toml content
            let content = DEFAULT_CARGO_TOML_CONTENT.replace("{$ui_name}", name);
            let _ = fs::write(cargo_toml.as_path(), &content)?;

            // write .ract
            let _ = fs::write(workspace_path.join(".ract").as_path(), "gen_ui")?;
            // if git is true, init git repository
            if git {
                git_init(workspace_path.as_path())?;
            }
            CreateLogs::Workspace.terminal().success();
            Ok(())
        }
        Err(e) => Err(e.to_string().into()),
    }
}

// create GenUI project depend on project info
fn create_gen_ui_project<P>(
    path: P,
    info: &ProjectInfo,
    underlayer: CompileTarget,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let workspace_path = path.as_ref().join(&info.name);
    // [LICENSE] -----------------------------------------------------------------------------------------
    info.write_license(workspace_path.as_path())?;
    // [use cargo to create a new project] ---------------------------------------------------------------
    std::process::Command::new("cargo")
        .arg("new")
        .arg("--bin")
        .arg(&info.name)
        .current_dir(workspace_path.as_path())
        .output()
        .map_or_else(
            |e| {
                TerminalLogger::new(e.to_string().as_str()).error();
                exit(2);
            },
            |out| {
                if out.status.success() {
                    CreateLogs::Cargo.terminal().success();
                    // [create dir: resources, views, components] ------------------------------------------
                    let ui_dir_path = workspace_path.join(&info.name);
                    for path in ["resources", "views", "components"].iter() {
                        let _ = fs::create_dir(ui_dir_path.join(path).as_path())?;
                    }
                    // [handle Cargo.toml] -----------------------------------------------------------------
                    let _ = info.write_gen_ui_cargo_toml(ui_dir_path.as_path())?;
                    // [create config files: gen_ui.toml, .gen_ui_cache] -----------------------------------
                    let _ = fs::create_new(ui_dir_path.join(".gen_ui_cache").as_path())?;
                    // - [gen_ui.toml] ---------------------------------------------------------------------
                    underlayer.write_gen_ui_toml(ui_dir_path.as_path())?;
                    // TODO! [create files in resources, views, components] --------------------------------
                    CreateLogs::Ui.terminal().success();
                    Ok(())
                } else {
                    Err(CreateLogs::CargoErr.to_string().into())
                }
            },
        )
}
