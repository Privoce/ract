// pub mod core;
mod app;
mod cli;
mod common;
mod entry;
mod log;
mod service;
// use core::run_cli;

use common::Result;
use entry::Language;
use log::TerminalLogger;
use ratatui::{
    crossterm::{
        event::DisableMouseCapture,
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    },
    DefaultTerminal,
};
use service::update::check_auto_update;

rust_i18n::i18n!("locales", fallback = ["en_US", "zh_CN"]);

fn destroy(terminal: &mut DefaultTerminal) -> Result<()> {
    ratatui::restore();
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<()> {
    // [check update] -----------------------------------------------------------------------------------
    match check_auto_update() {
        Ok(_) => {
            // [do init before cli and app run] -----------------------------------------------------------------
            let lang = Language::from_conf();
            let mut terminal = ratatui::init();
            let res = app::run(lang, &mut terminal);
            // [destroy terminal] -------------------------------------------------------------------------------
            destroy(&mut terminal)?;
            if let Err(e) = res {
                TerminalLogger::new(&e.to_string()).error();
            }
        }
        Err(e) => {
            TerminalLogger::new(&e.to_string()).error();
        }
    }

    Ok(())
}
