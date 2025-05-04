mod git;
mod rustc;
mod underlayer;

use gen_utils::error::Error;
use git::*;
use inquire::MultiSelect;
use rustc::*;
use std::process::exit;
use underlayer::*;

use crate::{
    entry::Tools,
    log::{InstallLogs, TerminalLogger},
};

use super::check::current_states;

pub fn run() {
    InstallLogs::Welcome.terminal().info();
    InstallLogs::Desc.terminal().info();

    // first use check to show user the current status
    let tools = match current_states() {
        Ok(tools) => {
            TerminalLogger::new(&format!("🔸 Current states:\n {}", tools)).info();
            tools
        }
        Err(e) => {
            TerminalLogger::new(&e.to_string()).error();
            exit(2);
        }
    };

    if let Err(e) = install_toolchain(tools) {
        TerminalLogger::new(&e.to_string()).error();
        exit(2);
    }
}

fn install_toolchain(tools: Tools) -> Result<(), Error> {
    MultiSelect::new("What tools you want to (re)install?", Tools::options())
        .with_default(&[2])
        .prompt()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |options| {
                // check and install the selected tools
                for option in options {
                    let res = match option {
                        "rustc|cargo" => {
                            InstallLogs::Install("rustc|cargo".to_string())
                                .terminal()
                                .info();
                            install_rustc()
                        }
                        "git" => {
                            InstallLogs::Install("git".to_string()).terminal().info();
                            install_git()
                        }
                        "makepad" => install_makepad(&tools),
                        _ => Err(Error::from("Unknown select option")),
                    };

                    if res.is_err() {
                        return res;
                    }
                }

                Ok(())
            },
        )
}
