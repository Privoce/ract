#[allow(unused)]
mod dashboard;
mod select;
mod state;
#[allow(unused)]
mod timeline;
#[allow(unused)]
pub mod unicode;
#[allow(unused)]
mod tab;
use crate::{
    cli::{
        command::{check::CheckCmd, config::ConfigCmd, init::InitCmd, Commands},
        Cli,
    },
    common::Result,
    entry::Language,
    service,
};
use clap::Parser;
pub use state::*;

// use crossterm::{
//     event::DisableMouseCapture,
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame, Terminal,
};
use std::time::Duration;

pub use dashboard::Dashboard;
pub use select::*;
pub use timeline::*;
pub use tab::*;

pub fn run(lang: Language, terminal: &mut DefaultTerminal) -> Result<()> {
    // [match cli command] ------------------------------------------------------------------------------
    let cmd = Cli::parse().commands;
    if let Commands::Init = cmd {
        InitCmd::new(lang).run(terminal, false)?;
    } else {
        match cmd {
            Commands::Check => {
                let cmd: CheckCmd = CheckCmd::before(&lang, terminal)?.into();
                cmd.run(terminal, false)?;
            }
            Commands::Config => {
                // let cmd: ConfigCmd = ConfigCmd::before(&lang, terminal)?.into();
                // cmd.run(terminal, false)?;
                ConfigCmd::new(lang).run(terminal, false)?; 
            }
            Commands::Uninstall => service::uninstall::run(),
            // Commands::Studio => {service::run::makepad::run();},
            _ => {}
        }
        // match cmd {
        //     Commands::Create(create_args) => create_args.run(),
        //     Commands::Check => check::run(),
        //     Commands::Install => install::run(),
        //     Commands::Run => run::run(),
        //     Commands::Init => {}
        //     Commands::Config => config::run(),
        //     Commands::Studio => run::makepad::studio::run(),
        //     Commands::Wasm(wasm_args) => wasm_args.run(),
        //     Commands::Pkg => package::run(),
        //     Commands::Add { name } => add::run(name),
        //     Commands::Update(args) => args.run(),
        //     Commands::Uninstall => uninstall::run(),
        // }
    }

    Ok(())
}

pub trait AppComponent {
    type Outupt;

    fn new(lang: Language) -> Self;
    fn run(self, terminal: &mut DefaultTerminal, quit: bool) -> Result<Self::Outupt>;
    fn handle_events(&mut self) -> Result<()>;
    fn render(&mut self, frame: &mut Frame);
    fn quit(&mut self) -> ();
}
