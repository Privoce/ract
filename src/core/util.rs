use std::path::Path;
use std::{env::current_exe, path::PathBuf};
use crate::core::{entry::RactToml, log::CreateLogs};
use gen_utils::common::{fs, ToToml};
use gen_utils::error::Error;

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
