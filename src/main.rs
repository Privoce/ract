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
    // [do init before cli and app run] -----------------------------------------------------------------
    let lang = Language::from_conf();
    let mut terminal = ratatui::init();
    // run_cli();
    let res = app::run(lang, &mut terminal);
    // [destroy terminal] -------------------------------------------------------------------------------
    destroy(&mut terminal)?;
    if let Err(e) = res {
        TerminalLogger::new(&e.to_string()).error();
    }
    Ok(())
}