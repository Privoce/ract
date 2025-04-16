#[allow(unused)]
mod dashboard;
#[allow(unused)]
mod timeline;

use crate::{
    cli::{
        command::{check::CheckCmd, init::InitCmd, Commands},
        Cli,
    },
    common::Result,
    entry::Language,
    service,
};
use clap::Parser;

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
pub use timeline::*;

pub fn run() -> Result<()> {
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

    // [do init before cli and app run] -----------------------------------------------------------------
    let lang = Language::from_conf();
    // [match cli command] ------------------------------------------------------------------------------
    let cmd = Cli::parse().commands;
    if let Commands::Init = cmd {
        let mut terminal = ratatui::init();
        InitCmd::new(lang).run(&mut terminal)?;
        destroy(&mut terminal)?;
    } else {
        match cmd {
            Commands::Check => {
                let cmd: CheckCmd = CheckCmd::before(&lang)?.into();
                let mut terminal = ratatui::init();
                cmd.run(&mut terminal)?;
                destroy(&mut terminal)?;
            }
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

// fn draw_app(frame: &mut Frame) {
//     let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
//     frame.render_widget(greeting, frame.area());
// }

// fn should_quit() -> Result<bool, Box<dyn std::error::Error>> {
//     if event::poll(Duration::from_millis(250))? {
//         if let Event::Key(key) = event::read()? {
//             return Ok(KeyCode::Char('q') == key.code);
//         }
//     }
//     Ok(false)
// }

/// ## Do before app run
/// 1. i18n
/// 2. ui init
pub fn before() -> Result<(DefaultTerminal, Language)> {
    Ok((ratatui::init(), Language::from_conf()))
}

pub fn after() {}

pub trait AppComponent {
    // fn before(&self) -> Result<()>{

    // }
    fn new(lang: Language) -> Self;
    fn run(self, terminal: &mut DefaultTerminal) -> Result<()>;
    fn handle_events(&mut self) -> Result<()>;
    fn quit(&mut self) -> ();
}

pub trait Component {
    fn new(lang: Language) -> Self;
}
