use std::{env::current_dir, path::Path, process::exit};

use gen_utils::{common::fs, error::Error};

use crate::core::log::{ProjectLogs, TerminalLogger};

pub mod gen_ui;
pub mod makepad;

pub fn run() {
    ProjectLogs::Welcome.terminal().rust();
    ProjectLogs::Desc.terminal().info();

    // get current dir path and check has .ract file
    let path = current_dir().unwrap();
    let ract_path = path.join(".ract");

    if !ract_path.exists() {
        ProjectLogs::Error(
            "Please make sure your project root has a `.ract` file to point the project kind"
                .to_string(),
        )
        .terminal()
        .error();
        return;
    } else {
        if let Err(e) = run_project(path, ract_path) {
            TerminalLogger::new(&e.to_string()).error();
            exit(2);
        }
    }
}

fn run_project<P>(path: P, ract_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let content = fs::read(ract_path.as_ref())?;

    match content.as_str() {
        "makepad" => makepad::origin::run(path.as_ref()),
        "gen_ui" => gen_ui::run(),
        _ => Err(ProjectLogs::Error("Invalid project kind".to_string())
            .to_string()
            .into()),
    }
}
