use clap::Parser;

use super::command::{check, config, init, install, Commands, run, package};

#[derive(Parser)]
#[command(
    name = "gpiler",
    about = "A build tool for Rust front-end framework GenUI",
    version = "0.1.0",
    author = "syf20020816@outlook.com"
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

impl Cli {
    pub fn match_cmd(&self) {
        if let Commands::Init = &self.commands {
            init::run();
            return;
        } else {
            init::check();
            match &self.commands {
                Commands::Create(create_args) => create_args.run(),
                Commands::Check => check::run(),
                Commands::Install => install::run(),
                Commands::Run => run::run(),
                Commands::Init => {}
                Commands::Config => config::run(),
                Commands::Studio => run::makepad::studio::run(),
                Commands::Wasm(wasm_args) => wasm_args.run(),
                Commands::Pkg => package::run(),
            }
        }
    }
}
