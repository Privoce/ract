use std::{env::current_dir, path::Path, process::exit};

use gen_utils::{
    common::fs,
    error::{Error, FsError},
};
use inquire::Text;
use toml_edit::DocumentMut;

use crate::core::log::{ProjectLogs, TerminalLogger, WasmLogs};
use clap::Args;
pub mod makepad;

#[derive(Args, Debug)]
pub struct WasmArgs {
    #[arg(short, long, default_value = None)]
    pub project: Option<String>,
}

impl WasmArgs {
    pub fn run(&self) -> () {
        WasmLogs::Welcome.terminal().rust();
        WasmLogs::Desc.terminal().info();
        // let user input the port
        let port = Text::new("Port for the web studio")
            .with_placeholder("The port should in range: [1 ~ 65535], recommend: [8010 ~ 65535]")
            .with_default("8010")
            .prompt()
            .map_or_else(
                |e| Err(Error::from(e.to_string())),
                |port| {
                    // validate the port
                    port.parse::<u16>()
                        .map_err(|_| Error::from("Invalid port!"))
                },
            )
            .map_err(|e| {
                TerminalLogger::new(&e.to_string()).error();
                exit(2);
            })
            .unwrap();

        let path = current_dir().unwrap();
        let _ = if let Some(project) = self.project.as_ref() {
            // do makepad run wasm
            makepad::run(path.as_path(), project, port)
        } else {
            // get current dir path and check has .gpiler file
            let gpiler_path = path.join(".gpiler");
            if !gpiler_path.exists() {
                ProjectLogs::Error(
        "Please make sure your project root has a `.gpiler` file to point the project kind"
            .to_string(),
    )
    .terminal()
    .error();
                Err(FsError::FileNotFound(gpiler_path).into())
            } else {
                run_wasm(path, gpiler_path, port)
            }
        }
        .map_err(|e| {
            TerminalLogger::new(&e.to_string()).error();
            exit(2);
        });
    }
}

fn run_wasm<P>(path: P, gpiler_path: P, port: u16) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let content = fs::read(gpiler_path.as_ref())?;
    // get project name from Cargo.toml
    let cargo_toml = fs::read(path.as_ref().join("Cargo.toml"))?
        .parse::<DocumentMut>()
        .map_err(|e| e.to_string())?;
    let project = cargo_toml["package"]["name"].as_str().unwrap().to_string();

    match content.as_str() {
        "makepad" => makepad::run(path.as_ref(), &project, port),
        _ => Err(ProjectLogs::Error("Invalid project kind".to_string())
            .to_string()
            .into()),
    }
}
