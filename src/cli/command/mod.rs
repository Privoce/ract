/// add a new plugin to the project
pub mod add;
/// help you to check the tool chain
pub mod check;
/// help you to config the cli and tool chain
pub mod config;
/// create a new project for GenUI
pub mod create;
/// help you to init the cli
pub mod init;
/// install the tool chain
pub mod install;
/// package project
pub mod package;
/// run the current project
pub mod run;
/// update the cli
pub mod update;
/// uninstall the cli
pub mod uninstall;
/// run wasm project
pub mod wasm;

use clap::Subcommand;
use create::CreateArgs;
use update::UpdateArgs;
use wasm::WasmArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Ract will build a **Makepad** or **GenUI** project template based on the configuration entered by the user
    Create(CreateArgs),
    /// Check if required tools and dependencies are installed. Options include: [Basic, Underlayer, All]
    Check,
    /// Install required tools and dependencies for development.  
    Install,
    /// Run **Makepad** or **GenUI** projects.
    Run,
    /// Initialize or reset the CLI. Ract will generate: [.env, chain/env.toml, chain/]
    Init,
    /// Set or update environment variables and CLI configurations.  
    Config,
    /// Start Makepad Studio for GUI projects.
    Studio,
    /// Build and run a WASM project directly from the CLI.
    Wasm(WasmArgs),
    /// Package a project using `cargo-packager`. (Currently only supports Makepad projects)
    Pkg,
    /// Add a new plugin to the project.
    Add { name: String },
    /// Update the CLI to the latest version.
    Update(UpdateArgs),
    /// Uninstall the CLI.
    Uninstall
}
