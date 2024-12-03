mod gen_ui;
mod makepad;

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use crate::core::{
    constant::DEFAULT_GITIGNORE,
    entry::{CompileTarget, ProjectInfo},
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

        // show info in the terminal: Create a new {target} project at {path}
        // if self.confirm_create() {
        //     let _ = self.create_gen_project(info, underlayer, git);
        // } else {
        //     // redo the project info
        //     return self.run();
        // }
    }
    fn create_project(&self) -> Result<(), Error> {
        match self.path.canonicalize() {
            Ok(path) => {
                return Select::new(
                    "Which project you want to create?",
                    vec!["makepad", "gen_ui"],
                )
                .with_starting_cursor(1)
                .prompt()
                .map_or_else(
                    |e| Err(e.to_string().into()),
                    |option| {
                        // first get the project info
                        let info = self.get_info();
                        let git = self.init_git();
                        if self.confirm_create() {
                            match option {
                                "makepad" => makepad::create(path.as_path(), info, git),
                                "gen_ui" => {
                                    // set project path, target underlayer ...
                                    let underlayer = self.get_underlayer();
                                    gen_ui::create(path.as_path(), info, underlayer, git)
                                }
                                _ => Err("Invalid project type".to_string().into()),
                            }
                        } else {
                            Err("You cancel the project creation".to_string().into())
                        }
                    },
                );
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

    fn get_underlayer(&self) -> CompileTarget {
        let underlayer = Select::new("Choose target underlayer: ", CompileTarget::options())
            .with_help_message("Choose the target underlayer for the project, default is Makepad")
            .prompt_skippable()
            .unwrap();

        underlayer.map_or(CompileTarget::Makepad, |underlayer| {
            underlayer.parse().unwrap()
        })
    }

    fn get_info(&self) -> ProjectInfo {
        let name = Text::new("Project name:")
            .with_placeholder("Your project name use snake_case")
            .prompt()
            .expect("Failed to get project name");

        let authors = Text::new("Authors name:")
            .with_placeholder("format: name <email> and use `,` to separate multiple authors")
            .prompt_skippable()
            .expect("Failed to get author name")
            .filter(|s| !s.is_empty());

        let description = Text::new("Project description:")
            .with_default(
                "This project is created by ract. Repo: https://github.com/Privoce/GenUI",
            )
            .prompt_skippable()
            .unwrap();

        let license = Select::new("Choose LICENSE:", License::options())
            .prompt()
            .expect("Failed to get license");

        let version = Text::new("Version:")
            .with_default("0.1.0")
            .with_placeholder("0.1.0")
            .prompt()
            .unwrap();

        let keywords = Text::new("Keywords:")
            .with_help_message("You can input multiple keywords, or press Enter to skip")
            .with_default("front_end, ui")
            .with_placeholder("gen_ui, front_end, ui")
            .prompt()
            .unwrap();

        if
        // confirm the project information
        Confirm::new("Do you confirm the project information?")
            .with_default(true)
            .with_help_message(
                "If you confirm, the project will be created with the above information",
            )
            .prompt()
            .expect("Failed to confirm project information")
        {
            let authors = authors.map(|authors| {
                authors
                    .split(',')
                    .map(|author| author.parse().unwrap())
                    .collect()
            });

            return ProjectInfo {
                name,
                version,
                authors,
                description,
                license: license.parse().unwrap(),
                keywords: keywords.split(',').map(|x| x.trim().to_string()).collect(),
            };
        } else {
            return self.get_info();
        }
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
