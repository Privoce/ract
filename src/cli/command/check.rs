use inquire::Select;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};
use std::{str::FromStr, time::Duration};

use crate::{
    app::{AppComponent, Dashboard},
    common::Result,
    entry::{Checks, Language},
    log::{error::Error, CheckLogs, LogExt, LogItem, LogType},
    service::{
        self,
        check::{check_basic, CheckItem},
    },
};

pub struct CheckCmd {
    state: CheckState,
    lang: Language,
    option: Checks,
    logs: Vec<LogItem>,
    items: Vec<CheckItem>,
}

impl AppComponent for CheckCmd {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            option: Checks::default(),
            logs: vec![],
            items: vec![],
        }
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        match self.state {
            CheckState::Start => {
                self.state = CheckState::Run;
            }
            CheckState::Run => {
                self.handle_running();
                self.state = CheckState::Pause;
            }
            CheckState::Pause => {}
            CheckState::Quit => {}
        }

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
        // [dashboard] ----------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        // [render items] ----------------------------------------------------------------------------------------------
        let (items, list_height): (Vec<ListItem>, u16) =
            self.items
                .iter()
                .fold((vec![], 0), |(mut items, mut height), item| {
                    let item: ListItem = item.into();
                    height += item.height() as u16;
                    items.push(item);

                    (items, height)
                });

        let msg = self.render_msg();
        let list = List::new(items);
        let [msg_area, dashboard_area] = Layout::vertical([
            Constraint::Length(msg.height() as u16),
            Constraint::Length(dashboard.height(list_height, 0)),
        ])
        .spacing(1)
        .vertical_margin(1)
        .areas(area);
        // [render components] ----------------------------------------------------------------------------------------------
        frame.render_widget(msg, msg_area);
        dashboard.render(frame, dashboard_area, |frame, area| {
            frame.render_widget(list, area);
        });
    }
    fn render_msg(&self) -> Text {
        let items: Vec<Line> = self.logs.iter().map(|log| log.fmt_line()).collect();
        Text::from_iter(items)
    }
    pub fn before(lang: &Language) -> Result<(Checks, &Language)> {
        Select::new(&CheckLogs::Select.t(lang).to_string(), Checks::options())
            .prompt()
            .map_or_else(
                |_| Err(Error::Other(CheckLogs::SelectFailed.t(lang).to_string())),
                |check| Ok((Checks::from_str(check).unwrap(), lang)),
            )
    }
    fn handle_running(&mut self) {
        match self.option {
            Checks::Basic => {
                let checks = check_basic();
                self.logs.extend(
                    checks
                        .iter()
                        .map(|item| (item, &self.lang).into())
                        .collect::<Vec<LogItem>>(),
                );
                self.items.extend(checks);
            }
            Checks::Underlayer => todo!(),
            Checks::All => todo!(),
        }
    }
}

impl From<(Checks, &Language)> for CheckCmd {
    fn from(value: (Checks, &Language)) -> Self {
        Self {
            state: Default::default(),
            lang: value.1.clone(),
            option: value.0,
            logs: vec![],
            items: vec![],
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
    pub fn is_start(&self) -> bool {
        matches!(self, CheckState::Start)
    }
}

#[cfg(test)]
mod syud {
    use ratatui::widgets::ListItem;

    use crate::service::check::CheckItem;

    #[test]
    fn height() {
        let (items, list_height): (Vec<ListItem>, u16) =
            vec![
                CheckItem::default(),
                CheckItem::default(),
                CheckItem::default(),
            ]
                .iter()
                .fold((vec![], 0), |(mut items, mut height), item| {
                    let item: ListItem = item.into();
                    height += item.height() as u16;
                    items.push(item);

                    (items, height)
                });

        dbg!(list_height);
    }
}
