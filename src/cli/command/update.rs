use clap::Args;
use colored::Colorize;
use gen_utils::{common::shadow_cmd, error::Error};
use inquire::Confirm;

use super::uninstall;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(short, long, default_value = "false")]
    pub force: bool,
}
