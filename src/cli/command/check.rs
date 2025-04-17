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
    app::{AppComponent, ComponentState, Dashboard, State},
    common::Result,
    entry::{Checks, Language},
    log::{error::Error, CheckLogs, LogExt, LogItem, LogType},
    service::{
        self,
        check::{check_basic, CheckItem},
    },
};

pub struct CheckCmd {
    state: ComponentState<CheckState>,
    lang: Language,
    option: Checks,
    logs: Vec<LogItem>,
    items: Vec<CheckItem>,
    cost: Option<Duration>,
}

impl AppComponent for CheckCmd {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            option: Checks::default(),
            logs: vec![],
            items: vec![],
            cost: None,
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
            ComponentState::Start => {
                self.state.next();
            }
            ComponentState::Run(r) => match r {
                CheckState::Basic => match self.option {
                    Checks::Basic | Checks::All => {
                        self.handle_running();
                        self.state = ComponentState::Pause;
                    }
                    Checks::Underlayer => {
                        // todo
                        self.state.next();
                    }
                },
                CheckState::Underlayer => todo!(),
            },
            ComponentState::Pause => {}
            ComponentState::Quit => {}
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
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        // [dashboard] ----------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        dashboard.cost = self.cost;
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
    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl CheckCmd {
    
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
                let start = std::time::Instant::now();
                let checks = check_basic();
                self.cost.replace(start.elapsed());
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
        self.logs
            .push(LogItem::info(CheckLogs::Complete.t(&self.lang).to_string()));
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
            cost: None,
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum CheckState {
    #[default]
    Basic,
    Underlayer,
}

impl State for CheckState {
    fn next(&mut self) -> () {
        match self {
            CheckState::Basic => {
                *self = CheckState::Underlayer;
            }
            CheckState::Underlayer => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, CheckState::Underlayer)
    }

    fn to_run_end(&mut self) -> () {
        *self = CheckState::Underlayer;
    }
}
