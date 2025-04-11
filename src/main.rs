// pub mod core;
mod app;
mod cli;
mod common;
mod entry;
mod log;
mod service;
// use core::run_cli;

use log::error::Error;
use common::Result;

rust_i18n::i18n!("locales", fallback = ["en_US", "zh_CN"]);

fn main() -> Result<()> {
    // run_cli();
    app::run()
}
