mod gen_ui;
mod makepad;

use std::{
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

use crate::core::{
    constant::DEFAULT_GITIGNORE,
    entry::{CompileTarget, FrameworkType, ProjectInfo, ProjectPackageType, RactToml},
    log::{CreateLogs, TerminalLogger},
};

use clap::Args;
use gen_utils::{common::fs, compiler::License, error::Error};
use inquire::{Confirm, Select, Text};

use super::check::current_states;

/// Create a new project
/// This command will create a new project at the specified path
/// ## Create a new project at the current directory
/// ```shell
/// ract create
/// ```
#[derive(Args, Debug)]
pub struct CreateArgs {
    // #[arg(short, long, default_value = "makepad")]
    // pub target: CompileTarget,
    /// Path to create the project
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
}

impl CreateArgs {
    /// create a new rust workspace project
    pub fn run(&self) {
        let _ = CreateLogs::Welcome.terminal().rust();

        // check state
        match current_states() {
            Ok(tool) => {
                TerminalLogger::new(&format!("🔸 Current states:\n {}", tool)).info();
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
        // [select framework] ---------------------------------------------------------
        let framwork = Select::new(
            "Which framework template do you want to create?",
            FrameworkType::options(),
        )
        .with_starting_cursor(0)
        .prompt()
        .unwrap();
        let framework = FrameworkType::from_str(&framwork)?;
        // [get project info] ---------------------------------------------------------
        let project = ProjectPackageType::new(framework)?;

        let ract_toml = match framework {
            FrameworkType::GenUI => RactToml::gen_ui(project.members().unwrap()),
            FrameworkType::Makepad => RactToml::makepad(),
        };

        println!("{}", ract_toml);
        Ok(())
        // match self.path.canonicalize() {
        //     Ok(path) => {
        //         return Select::new(
        //             "Which framework template do you want to create?",
        //             vec!["makepad", "gen_ui"],
        //         )
        //         .with_starting_cursor(1)
        //         .prompt()
        //         .map_or_else(
        //             |e| Err(e.to_string().into()),
        //             |option| {
        //                 // first get the project info
        //                 let info = self.get_info();
        //                 let git = self.init_git();
        //                 if self.confirm_create() {
        //                     match option {
        //                         "makepad" => makepad::create(path.as_path(), info, git),
        //                         "gen_ui" => {
        //                             // set project path, target underlayer ...
        //                             let underlayer = self.get_underlayer();
        //                             gen_ui::create(path.as_path(), info, underlayer, git)
        //                         }
        //                         _ => Err("Invalid project type".to_string().into()),
        //                     }
        //                 } else {
        //                     Err("You cancel the project creation".to_string().into())
        //                 }
        //             },
        //         );
        //     }
        //     Err(e) => Err(e.to_string().into()),
        // }
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

    fn get_underlayer(&self) -> CompileTarget {
        let underlayer = Select::new("Choose target underlayer: ", CompileTarget::options())
            .with_help_message("Choose the target underlayer for the project, default is Makepad")
            .prompt_skippable()
            .unwrap();

        underlayer.map_or(CompileTarget::Makepad, |underlayer| {
            underlayer.parse().unwrap()
        })
    }
}

pub fn git_init<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // init git repository
    std::process::Command::new("git")
        .arg("init")
        .current_dir(path.as_ref())
        .output()
        .map_or_else(
            |e| Err(Error::from(e.to_string())),
            |out| {
                if out.status.success() {
                    // write .gitignore
                    let _ = fs::write(
                        path.as_ref().join(".gitignore").as_path(),
                        DEFAULT_GITIGNORE,
                    );
                    CreateLogs::Git.terminal().success();
                    Ok(())
                } else {
                    Err(CreateLogs::GitErr.to_string().into())
                }
            },
        )
}
