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
    cli::command::{
        check::CheckCmd,
        config::ConfigCmd,
        init::InitCmd,
        install::{InstallCmd, InstallCmdFollowUp},
        studio::StudioCmd,
        uninstall::UninstallCmd,
        wasm::WasmCmd,
        Commands,
    },
    common::Result,
    entry::Language,
    service::{self, package},
};
pub use state::*;

use ratatui::{
    crossterm::{
        event::DisableMouseCapture,
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    },
    DefaultTerminal, Frame,
};

pub use dashboard::Dashboard;
pub use list::*;
pub use select::*;
pub use tab::*;
pub use timeline::*;

/// # Run app
/// ## Return
/// - `true` do not need to do destroy
/// -
pub fn run(cmd: Commands, terminal: &mut Option<DefaultTerminal>) -> Result<()> {
    // [do init before cli and app run] -------------------------------------------------------------
    let lang = Language::from_conf();
    // [need init? and run] -------------------------------------------------------------------------
    if let Some(terminal) = terminal.as_mut() {
        let mut destroy_before = false;
        match cmd {
            Commands::Init => {
                InitCmd::new(lang).run(terminal, false)?;
            }
            Commands::Check => {
                CheckCmd::new(lang).run(terminal, false)?;
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
                WasmCmd::from((wasm_args, lang)).run(terminal, false)?;
            }
            Commands::Install => {
                let options = InstallCmd::new(lang).run(terminal, false)?;
                // do destroy before follow up
                destroy(terminal)?;
                options.follow_up(lang)?;
                destroy_before = true;
            }
            _ => {}
        }
        // [destroy terminal] -------------------------------------------------------------------------------
        if !destroy_before {
            destroy(terminal)?;
        }
    } else {
        // [do not need ratatui init or destroy] ------------------------------------------------------------
        match cmd {
            Commands::Update(args) => {
                args.run();
            }
            Commands::Pkg => {
                package::run(lang);
            }
            Commands::Create(create_args) => {
                create_args.run(lang);
            }
            Commands::Run => {
                service::run::run(lang);
            }
            Commands::Add { name } => {
                service::add::run(&name);
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn destroy(terminal: &mut DefaultTerminal) -> Result<()> {
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

#[allow(unused)]
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
