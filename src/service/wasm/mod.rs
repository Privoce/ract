use crate::{entry::RactToml, log::ProjectLogs};
use clap::Args;
use gen_utils::{
    common::{fs, ToToml},
    error::Error,
};
use std::{path::Path, process::Child};
use toml_edit::DocumentMut;
pub mod makepad;

#[derive(Args, Debug)]
pub struct WasmArgs {
    #[arg(short, long, default_value = None)]
    pub project: Option<String>,
}

pub fn run_wasm<P>(path: P, ract_path: P, port: u16) -> Result<Child, Error>
where
    P: AsRef<Path>,
{
    fn get_project<P>(path: P) -> Result<String, Error>
    where
        P: AsRef<Path>,
    {
        // get project name from Cargo.toml
        let cargo_toml = fs::read(path.as_ref().join("Cargo.toml"))?
            .parse::<DocumentMut>()
            .map_err(|e| e.to_string())?;
        Ok(cargo_toml["package"]["name"].as_str().unwrap().to_string())
    }

    let ract_toml: RactToml = (&RactToml::read(ract_path.as_ref())?).try_into()?;

    match ract_toml.target {
        crate::entry::FrameworkType::GenUI => {
            if let Some(compiles) = ract_toml.compiles() {
                let member = compiles[0];
                let compiled_path = path.as_ref().join(member.target.as_path());
                let project = get_project(compiled_path.as_path())?;
                makepad::run(path.as_ref(), &project, port)
            } else {
                Err(Error::from(
                    ProjectLogs::Error("can not find compile target(s)!".to_string()).to_string(),
                ))
            }
        }
        crate::entry::FrameworkType::Makepad => {
            let project = get_project(path.as_ref())?;
            makepad::run(path.as_ref(), &project, port)
        }
    }
}
