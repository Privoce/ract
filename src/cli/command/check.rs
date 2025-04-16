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
    entry::{Checks, Language},
    log::{CheckLogs, LogExt}, service,
};

pub struct CheckCmd {
    state: CheckState,
    lang: Language,
}

impl AppComponent for CheckCmd {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
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
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
    }
    fn before(&self) {
        let check = Select::new(
            &CheckLogs::Select.t(&self.lang).to_string(),
            Checks::options(),
        )
        .prompt()
        .expect("select check failed");

        
        match Checks::from_str(check).unwrap() {
            Checks::Basic => {
                service::check::check_basic();
            }
            Checks::Underlayer => {
                service::check::check_underlayer();
            }
            Checks::All => {
                service::check::check_basic();
                service::check::check_underlayer();
            }
        };
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
