use std::path::{Path, PathBuf};

use gen_utils::{
    common::{fs, ToToml},
    error::Error,
};

use crate::core::{
    constant::{COMPONENT_MOD_GEN, EASY_GEN, GENUI_README, HELLO_GEN, HOME_GEN, ROOT_GEN, VIEW_MOD_GEN},
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
    // [create README.md] ----------------------------------------------------------
    let _ = fs::write(path.as_path().join("README.md"), GENUI_README)?;
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
                    let ui_dir_path = path.as_ref().join(&info.name);
                    // [clear main.rs] ---------------------------------------------------------------------
                    let _ = fs::write(ui_dir_path.join("src").join("main.rs"), "")?;
                    // [create dir: resources, views, components] ------------------------------------------
                    for path in ["resources", "views", "components"].iter() {
                        let _ = fs::create_dir(ui_dir_path.join(path))?;
                    }
                    // [create basic gen files: views/mod.gen, views/root.gen] -----------------------------
                    for (dir, file, content) in vec![
                        ("views", "mod.gen", VIEW_MOD_GEN),
                        ("views", "root.gen", ROOT_GEN),
                        ("views", "home.gen", HOME_GEN),
                        ("components", "mod.gen", COMPONENT_MOD_GEN),
                        ("components", "easy.gen", EASY_GEN),
                        ("components", "hello.gen", HELLO_GEN),
                    ] {
                        let _ = fs::write(ui_dir_path.join(dir).join(file), content)?;
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
