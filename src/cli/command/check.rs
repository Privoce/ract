use inquire::Select;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::{Line, Text},
    DefaultTerminal, Frame,
};
use std::{str::FromStr, time::Duration};

use crate::{
    app::AppComponent,
    common::Result,
    entry::{Checks, Language},
    log::{error::Error, CheckLogs, LogExt},
    service,
};

pub struct CheckCmd {
    state: CheckState,
    lang: Language,
    option: Checks,
}

impl AppComponent for CheckCmd {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            option: Checks::default(),
        }
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> crate::common::Result<()> {
        // self.before();

        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') | event::KeyCode::Enter => {
                            self.quit()
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl CheckCmd {
    pub fn option(&mut self, option: Checks) -> &mut Self {
        self.option = option;
        self
    }
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
    }
    pub fn before(lang: &Language) -> Result<(Checks, &Language)> {
        Select::new(&CheckLogs::Select.t(lang).to_string(), Checks::options())
            .prompt()
            .map_or_else(
                |_| Err(Error::Other(CheckLogs::SelectFailed.t(lang).to_string())),
                |check| Ok((Checks::from_str(check).unwrap(), lang)),
            )
    }
}

impl From<(Checks, &Language)> for CheckCmd {
    fn from(value: (Checks, &Language)) -> Self {
        Self {
            state: Default::default(),
            lang: value.1.clone(),
            option: value.0,
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum CheckState {
    #[default]
    Start,
    Run,
    Pause,
    Quit,
}

impl CheckState {
    pub fn is_quit(&self) -> bool {
        matches!(self, CheckState::Quit)
    }
    pub fn quit(&mut self) {
        *self = CheckState::Quit;
    }
}

#[cfg(test)]
mod t{
    use crate::{entry::Language, log::{CheckLogs, LogExt}};

    #[test]
    fn a(){
        let lang = Language::Zh;
       dbg!(CheckLogs::Select.t(&lang).to_string());
    }
}