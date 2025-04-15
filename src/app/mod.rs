#[allow(unused)]
mod dashboard;
#[allow(unused)]
mod timeline;

use crate::{
    cli::{command::init::InitCmd, command::Commands, Cli},
    common::Result,
    entry::Language,
};
use clap::Parser;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame, Terminal,
};
use std::time::Duration;

pub use dashboard::Dashboard;
pub use timeline::*;

pub fn run() -> Result<()> {
    // [do init before cli and app run] -----------------------------------------------------------------
    let (mut terminal, lang) = before()?;
    // [match cli command] ------------------------------------------------------------------------------
    let cmd = Cli::parse().commands;
    if let Commands::Init = cmd {
        // init::run();
        let init_cmd = InitCmd::new(lang);
        init_cmd.run(&mut terminal)?;

        // let mut dashboard = dashboard::Dashboard::new(lang);
        // dashboard.run(&mut terminal)?;
        ratatui::restore();
    } else {
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

    // loop {
    //     tm.draw(draw_app)?;
    //     if should_quit()? {
    //         break;
    //     }
    // }

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