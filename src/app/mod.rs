#[allow(unused)]
mod dashboard;
#[allow(unused)]
mod list;
mod select;
mod state;
#[allow(unused)]
mod tab;
#[allow(unused)]
mod timeline;
#[allow(unused)]
pub mod unicode;

use crate::{
    cli::{
        command::{
            check::CheckCmd, config::ConfigCmd, init::InitCmd, install::InstallCmd, studio::StudioCmd, uninstall::UninstallCmd, Commands
        },
        Cli,
    },
    common::Result,
    entry::Language,
    service::{self, package},
};
use clap::Parser;
pub use state::*;

use ratatui::{DefaultTerminal, Frame};

pub use dashboard::Dashboard;
pub use select::*;
pub use tab::*;
pub use timeline::*;

pub use list::*;

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
                ConfigCmd::new(lang).run(terminal, false)?;
            }
            Commands::Uninstall => {
                UninstallCmd::new(lang).run(terminal, false)?;
            }
            Commands::Studio => {
                StudioCmd::new(lang).run(terminal, false)?;
            }
            Commands::Wasm(wasm_args) => {
                wasm_args.run(&lang);
            }
            Commands::Update(args) => {
                args.run();
            }
            Commands::Pkg => {
                package::run();
            }
            Commands::Create(create_args) => {
                create_args.run();
            }
            Commands::Run => {
                service::run::run();
            }
            Commands::Install => {
                // service::install::run();
                InstallCmd::new(lang).run(terminal, false)?;
            }
            Commands::Add { name } => {
                service::add::run(&name);
            }
            _ => {}
        }
    }

    Ok(())
}

pub trait AppComponent {
    type Output;
    type State;
    /// # Create a new app instance
    fn new(lang: Language) -> Self;
    /// # Run the app
    fn run(mut self, terminal: &mut DefaultTerminal, quit: bool) -> Result<Self::Output>
    where
        Self: Sized,
        Self::State: State,
        Self::Output: Default,
    {
        while !self.state().is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if quit && self.state().is_pause() {
                self.quit();
            }
        }
        Ok(Self::Output::default())
    }
    fn handle_events(&mut self) -> Result<()>;
    fn render(&mut self, frame: &mut Frame);
    fn state(&self) -> &ComponentState<Self::State>
    where
        Self::State: State;
    /// # Quit the app
    fn quit(&mut self) -> ();
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputMode {
    Edit,
    #[default]
    Normal,
}

impl InputMode {
    pub fn next(&mut self) -> () {
        match self {
            InputMode::Edit => {
                *self = InputMode::Normal;
            }
            InputMode::Normal => {
                *self = InputMode::Edit;
            }
        }
    }

    pub fn is_edit(&self) -> bool {
        matches!(self, InputMode::Edit)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, InputMode::Normal)
    }
}
