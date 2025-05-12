use std::{path::PathBuf, process::exit, str::FromStr};

use crate::{
    entry::{FrameworkType, ProjectInfoType},
    log::{CreateLogs, LogItem, TerminalLogger},
};

use clap::Args;
use gen_utils::error::Error;
use inquire::{Confirm, Select};

use super::check::current_states;

/// ## Create a new project at the current directory
/// 
/// Create a new project
/// This command will create a new project at the specified path
/// 
/// ```shell
/// ract create
/// ```
#[derive(Args, Debug)]
pub struct CreateArgs {
    // #[arg(short, long, default_value = "makepad")]
    // pub target: Underlayer,
    /// Path to create the project
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
}

impl CreateArgs {
    /// create a new rust workspace project
    pub fn run(&self) {
        // check state
        match current_states() {
            Ok(tool) => {
                // TerminalLogger::new(&format!("{}", tool)).info();
                LogItem::info(format!("{}", tool)).multi().log();
                let is_ok = tool.is_ok();
                if !is_ok {
                    TerminalLogger::new("🔸 Current toolchain is not supported! You should use `ract install` to install toolchain or use `ract config` to set env").warning();
                    exit(2);
                }
                self.create_project().map_or_else(
                    |e| {
                        TerminalLogger::new(&e.to_string()).error();
                        exit(2);
                    },
                    |_| {
                        CreateLogs::Confirm.terminal().success();
                    },
                )
            }
            Err(e) => {
                TerminalLogger::new(&e.to_string()).error();
                exit(2);
            }
        }
    }
    fn create_project(&self) -> Result<(), Error> {
        match self.path.canonicalize() {
            Ok(path) => {
                // [select framework] ----------------------------------------------------------------
                let framwork = Select::new(
                    "Which framework template do you want to create?",
                    FrameworkType::options(),
                )
                .with_starting_cursor(0)
                .prompt()
                .map_err(|e| Error::from(e.to_string()))?;
                let framework = FrameworkType::from_str(&framwork)?;
                // [get project info] ----------------------------------------------------------------
                let project_info_type = ProjectInfoType::new(framework)?;
                // [get generate] --------------------------------------------------------------------
                let mut generator = project_info_type.create(path.as_path(), framework);
                // [init git repository] -------------------------------------------------------------
                generator.git = self.init_git();
                if self.confirm_create() {
                    // [do create] -------------------------------------------------------------------
                    generator.generate()
                } else {
                    CreateLogs::Cancel.terminal().warning();
                    return self.create_project();
                }
            }
            Err(e) => Err(e.to_string().into()),
        }
    }

    fn init_git(&self) -> bool {
        Confirm::new("Init as a git repository?")
            .with_default(true)
            .with_help_message(
                "If you confirm, the project will be initialized with a git repository",
            )
            .prompt()
            .expect("Failed to confirm git repository")
    }

    fn confirm_create(&self) -> bool {
        Confirm::new("Confirm All?")
            .with_default(true)
            .with_help_message("If you confirm, the project will be created with the above")
            .prompt()
            .expect("Failed to confirm project information")
    }
}
