use std::{env::current_dir, path::Path, process::{exit, Child}};

use gen_utils::{
    common::{fs, ToToml},
    error::{Error, FsError},
};
use inquire::Text;
use toml_edit::DocumentMut;

use crate::{
    entry::{Language, RactToml},
    log::{LogExt, ProjectLogs, TerminalLogger, WasmLogs},
};
use clap::Args;
pub mod makepad;

#[derive(Args, Debug)]
pub struct WasmArgs {
    #[arg(short, long, default_value = None)]
    pub project: Option<String>,
}

// impl WasmArgs {
//     pub fn run(&self, lang: &Language) -> () {
//         WasmLogs::Desc.terminal(lang).info();
//         // let user input the port
//         let port = Text::new(&WasmLogs::Port.t(lang).to_string())
//             .with_placeholder(&WasmLogs::Placeholder.t(lang).to_string())
//             .with_default("8010")
//             .prompt()
//             .map_or_else(
//                 |e| Err(Error::from(e.to_string())),
//                 |port| {
//                     // validate the port
//                     port.parse::<u16>()
//                         .map_err(|_| Error::from("Invalid port!"))
//                 },
//             )
//             .map_err(|e| {
//                 TerminalLogger::new(&e.to_string()).error();
//                 exit(2);
//             })
//             .unwrap();

//         let path = current_dir().unwrap();
//         let _ = if let Some(project) = self.project.as_ref() {
//             // do makepad run wasm
//             makepad::run(path.as_path(), project, port)
//         } else {
//             // get current dir path and check has .ract file
//             let ract_path = path.join(".ract");
//             if !ract_path.exists() {
//                 WasmLogs::NoRactConf.terminal(lang).error();
//                 Err(FsError::FileNotFound(ract_path).into())
//             } else {
//                 run_wasm(path, ract_path, port)
//             }
//         }
//         .map_err(|e| {
//             TerminalLogger::new(&e.to_string()).error();
//             exit(2);
//         });
//     }
// }

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
