use std::{env::current_dir, path::Path, process::exit};

use gen_utils::{common::ToToml, error::Error};

use crate::{
    entry::RactToml,
    log::{ProjectLogs, TerminalLogger},
};

pub mod gen_ui;
pub mod makepad;

pub fn run() {
    ProjectLogs::Welcome.terminal().info();
    ProjectLogs::Desc.terminal().info();

    // get current dir path and check has .ract file
    let path = current_dir().unwrap();

    if let Err(e) = run_project(path) {
        TerminalLogger::new(&e.to_string()).error();
        exit(2);
    }
}

fn run_project<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let ract_toml: RactToml = (&RactToml::read(path.as_ref().join(".ract"))?).try_into()?;

    match &ract_toml.target {
        crate::entry::FrameworkType::GenUI => gen_ui::run(path.as_ref(), &ract_toml),
        crate::entry::FrameworkType::Makepad => makepad::run(path.as_ref()),
    }
}
