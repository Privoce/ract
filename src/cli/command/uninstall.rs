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

pub struct UninstallCmd {
    lang: Language,
    state: ComponentState<BaseRunState>,
    log: Log,
    selected: bool,
}

impl AppComponent for UninstallCmd {
    type Outupt = ();

    fn new(lang: Language) -> Self {
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            selected: false,
        }
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal, quit: bool) -> Result<Self::Outupt> {
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
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up | event::KeyCode::Down => {
                            self.selected = !self.selected;
                        }
                        event::KeyCode::Enter => {
                            if self.selected {
                                match uninstall_all() {
                                    Ok(_) => {
                                        self.log.push(LogItem::success(
                                            UninstallLogs::Success("Ract".to_string())
                                                .t(&self.lang)
                                                .to_string(),
                                        ));
                                        self.quit();
                                    }
                                    Err(e) => {
                                        self.log.push(LogItem::error(
                                            UninstallLogs::Failed {
                                                name: "Ract".to_string(),
                                                reason: Some(e.to_string()),
                                            }
                                            .t(&self.lang)
                                            .to_string(),
                                        ));
                                    }
                                }
                            } else {
                                self.quit();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let _ = Confirm::new(
            &UninstallLogs::Select("Ract".to_string())
                .t(&self.lang)
                .to_string(),
            self.lang,
        )
        .0
        .selected(if self.selected { 0 } else { 1 })
        .render_from(area, frame);
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}
