// create a new project for GenUI
mod create;
// help you to check the tool chain
pub mod check;
/// help you to config the cli and tool chain
pub mod config;
/// help you to init the cli
pub mod init;
/// install the tool chain
pub mod install;
/// run the current project
pub mod run;
/// run wasm project
pub mod wasm;
/// package project
pub mod package;


use clap::Subcommand;
use create::CreateArgs;
use wasm::WasmArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project [CreateArgs]
    Create(CreateArgs),
    /// Check toolchain
    Check,
    /// Install toolchain
    Install,
    /// Run the current project
    Run,
    /// Init or reset the cli
    Init,
    /// do config for the cli
    Config,
    /// makepad studio
    Studio,
    /// run wasm project
    Wasm(WasmArgs),
    /// package project
    Pkg,
}
