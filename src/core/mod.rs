mod cli;
mod command;
mod entry;
pub mod util;
mod log;
/// default constants or templates
mod constant;

use clap::Parser;
use cli::Cli;

pub fn run_cli() {
    let cli = Cli::parse();
    cli.match_cmd();
}
