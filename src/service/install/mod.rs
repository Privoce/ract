mod git;
mod rustc;
mod underlayer;

use gen_utils::error::Error;
pub use git::*;
use inquire::MultiSelect;
pub use rustc::*;
use std::process::exit;
pub use underlayer::*;

use crate::{
    cli::command::install::InstallOptions,
    common::is_empty_dir,
    entry::{ChainEnvToml, Tools},
    log::{InstallLogs, TerminalLogger},
};

use super::check::current_states;

// pub fn run() {
//     // InstallLogs::Welcome.terminal().info();
//     InstallLogs::Desc.terminal().info();

//     // first use check to show user the current status
//     let tools = match current_states() {
//         Ok(tools) => {
//             TerminalLogger::new(&format!("🔸 Current states:\n {}", tools)).info();
//             tools
//         }
//         Err(e) => {
//             TerminalLogger::new(&e.to_string()).error();
//             exit(2);
//         }
//     };

//     if let Err(e) = install_toolchain(tools) {
//         TerminalLogger::new(&e.to_string()).error();
//         exit(2);
//     }
// }

// fn install_toolchain(tools: Tools) -> Result<(), Error> {
//     MultiSelect::new("What tools you want to (re)install?", Tools::options())
//         .with_default(&[2])
//         .prompt()
//         .map_or_else(
//             |e| Err(e.to_string().into()),
//             |options| {
//                 // check and install the selected tools
//                 for option in options {
//                     let res = match option {
//                         "rustc|cargo" => {
//                             InstallLogs::Install("rustc|cargo".to_string())
//                                 .terminal()
//                                 .info();
//                             install_rustc()
//                         }
//                         "git" => {
//                             InstallLogs::Install("git".to_string()).terminal().info();
//                             install_git()
//                         }
//                         "makepad" => install_makepad(&tools),
//                         _ => Err(Error::from("Unknown select option")),
//                     };

//                     if res.is_err() {
//                         return res;
//                     }
//                 }

//                 Ok(())
//             },
//         )
// }

pub fn run(options: InstallOptions) -> Result<(), Error> {
    let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let path = chain_env_toml.chain_path();
    // first you must install makepad (so check and ask user need to update?)
    let makepad_widgets_path = chain_env_toml.makepad_widgets_path();
    let makepad_is_ok = !is_empty_dir(makepad_widgets_path)?;
    // install depend on options
    for option in options {
        match option {
            Tools::Basic(basic_tools) => match basic_tools {
                crate::entry::BasicTools::Ructc => {
                    install_rustc()?;
                }
                crate::entry::BasicTools::Git => {
                    install_git()?;
                }
            },
            Tools::Underlayer(underlayer_tools) => match underlayer_tools {
                crate::entry::UnderlayerTools::Makepad(makepad_tools) => {
                    underlayer::install(path.as_path(), makepad_is_ok, makepad_tools)?;
                }
            },
        }
    }
    Ok(())
}
