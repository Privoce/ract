
use crate::{entry::RactToml, log::CreateLogs};
use gen_utils::common::{fs, ToToml};
use gen_utils::error::Error;
use std::path::Path;
use std::{env::current_exe, path::PathBuf};
use toml_edit::DocumentMut;

pub fn exe_path() -> Result<PathBuf, Error> {
    let mut path = current_exe().map_err(|e| Error::from(e.to_string()))?;
    path.pop();
    Ok(path)
}

/// ## create a rust workspace project
/// - create a empty workspace dir
/// - create a Cargo.toml
/// - create a .ract file
pub fn create_workspace<P>(path: P, cargo_toml: &str, ract_toml: &RactToml) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // [create workspace project] --------------------------------------------------------------------
    match std::fs::create_dir(path.as_ref()) {
        Ok(_) => {
            // [create Cargo.toml] -------------------------------------------------------------------
            let _ = fs::write(path.as_ref().join("Cargo.toml"), cargo_toml)?;
            // [write .ract] -------------------------------------------------------------------------
            ract_toml.write(path.as_ref().join(".ract"))?;
            CreateLogs::Workspace.terminal().success();
            Ok(())
        }
        Err(e) => Err(e.to_string().into()),
    }
}

// judget current path is rust workspace or not
pub fn is_workspace<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    fn handle<P>(path: P, mut count: usize) -> Result<bool, Error>
    where
        P: AsRef<Path>,
    {
        count += 1;
        if count > 2 {
            return Ok(false);
        }
        let cargo_toml = path.as_ref().join("Cargo.toml");
        let toml = fs::read(cargo_toml)?
            .parse::<DocumentMut>()
            .map_err(|e| e.to_string())?;
        if toml.get("workspace").is_some() {
            return Ok(true);
        } else {
            let pre_path = path
                .as_ref()
                .parent()
                .ok_or_else(|| Error::from("can not get parent path"))?;
            // handle
            return handle(pre_path, count);
        }
    }

    handle(path, 0).unwrap_or(false)
}

pub fn is_empty_dir<P>(path: Option<P>) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    if let Some(path) = path {
        if fs::exists_dir(path.as_ref()) {
            let mut entries = std::fs::read_dir(path).map_err(|e| Error::from(e.to_string()))?;
            return Ok(entries.next().is_none());
        }
    }

    Ok(true)
}