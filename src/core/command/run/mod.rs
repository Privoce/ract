use std::{env::current_dir, path::Path, process::exit};

use gen_utils::{common::fs, error::Error};

use crate::core::log::{ProjectLogs, TerminalLogger};

pub mod gen_ui;
pub mod makepad;

pub fn run() {
    ProjectLogs::Welcome.terminal().rust();
    ProjectLogs::Desc.terminal().info();

    // get current dir path and check has .gpiler file
    let path = current_dir().unwrap();
    let gpiler_path = path.join(".gpiler");

    if !gpiler_path.exists() {
        ProjectLogs::Error(
            "Please make sure your project root has a `.gpiler` file to point the project kind"
                .to_string(),
        )
        .terminal()
        .error();
        return;
    } else {
        if let Err(e) = run_project(path, gpiler_path) {
            TerminalLogger::new(&e.to_string()).error();
            exit(2);
        }
    }
}

fn run_project<P>(path: P, gpiler_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let content = fs::read(gpiler_path.as_ref())?;

    match content.as_str() {
        "makepad" => makepad::origin::run(path.as_ref()),
        "gen_ui" => gen_ui::run(),
        _ => Err(ProjectLogs::Error("Invalid project kind".to_string())
            .to_string()
            .into()),
    }
}
