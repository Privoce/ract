pub mod command;
use clap::Parser;
use command::Commands;

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
