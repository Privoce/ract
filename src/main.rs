// pub mod core;
mod app;
mod cli;
mod common;
mod entry;
mod log;
mod service;
// use core::run_cli;

use app::destroy;
use clap::Parser;
use cli::Cli;
use common::Result;
use log::TerminalLogger;
use service::update::check_auto_update;
rust_i18n::i18n!("locales", fallback = ["en_US", "zh_CN"]);

fn main() -> Result<()> {
    // [check update] -----------------------------------------------------------------------------------
    match check_auto_update() {
        Ok(_) => {
            // [read from terminal] ---------------------------------------------------------------------
            let cmd = Cli::parse().commands;
            let mut terminal = if cmd.need_init() {
                Some(ratatui::init())
            } else {
                None
            };

            let res = app::run(cmd, &mut terminal);
            // [error handling] -------------------------------------------------------------------------
            if let Err(e) = res {
                TerminalLogger::new(&e.to_string()).error();
                if let Some(terminal) = terminal.as_mut() {
                    destroy(terminal)?;
                }
            }
        }
        Err(e) => {
            TerminalLogger::new(&e.to_string()).error();
        }
    }
    Ok(())
}
