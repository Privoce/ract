use crate::{
    app::{AppComponent, BaseRunState, ComponentState, Confirm},
    common::Result,
    entry::Language,
    log::{Log, LogExt, LogItem, UninstallLogs},
    service::uninstall::uninstall_all,
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    Frame,
};
use std::time::Duration;

pub struct StudioCmd {
    lang: Language,
    state: ComponentState<BaseRunState>,
    log: Log,

}

impl AppComponent for StudioCmd {
    type Output = ();
    type State = BaseRunState;
    fn new(lang: Language) -> Self {
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
        }
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal, quit: bool) -> Result<Self::Output> {
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if quit && self.state.is_pause() {
                self.quit();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        todo!()
    }

    fn render(&mut self, frame: &mut Frame) {
        todo!()
    }

    fn quit(&mut self) -> () {
        todo!()
    }
    
    fn state(&self) -> &ComponentState<Self::State> {
        todo!()
    }
    
   
}