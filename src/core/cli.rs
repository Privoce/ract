use clap::Parser;

use super::command::{add, check, config, init, install, package, run, uninstall, Commands};

#[derive(Parser)]
#[command(
    name = "ract",
    about = "Ract is a conversational CLI tool written in Rust, providing an all-in-one solution for integrating dependencies, setting up environments, generating project templates, running, and packaging projects with frameworks like GenUI and Makepad. Simplify your development workflow with minimal arguments and intuitive dialogs. 🚀",
    version = "0.1.8",
    author = "Will SHENG<syf20020816@outlook.com>"
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
                Commands::Add{name} => add::run(name),
                Commands::Update(args) => args.run(),
                Commands::Uninstall => uninstall::run(),
            }
        }
    }
}
