use inquire::Select;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    text::{Line, Text},
    widgets::{List, ListItem},
    Frame,
};
use std::{str::FromStr, time::Duration};

use crate::{
    app::{AppComponent, ComponentState, Dashboard, State},
    common::Result,
    entry::{Checks, Language, Underlayer},
    log::{error::Error, CheckLogs, Log, LogExt, LogItem, LogType},
    service::{
        self,
        check::{check_basic, CheckItem},
    },
};

pub struct CheckCmd {
    state: ComponentState<CheckState>,
    lang: Language,
    option: Checks,
    log: Log,
    items: Vec<CheckItem>,
    cost: Option<Duration>,
}

impl AppComponent for CheckCmd {
    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            option: Checks::default(),
            log: Log::new(),
            items: vec![],
            cost: None,
        }
    }

    fn run(
        mut self,
        terminal: &mut ratatui::DefaultTerminal,
        quit: bool,
    ) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if quit && self.state.is_pause() {
                self.quit();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        match self.state {
            ComponentState::Start => {
                self.state.next();
            }
            ComponentState::Run(r) => self.handle_running(r),
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
        // dashboard.render(frame, dashboard_area, |frame, area| {
        //     frame.render_widget(list, area);
        // });
    }
    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl CheckCmd {
    fn render_msg(&self) -> Text {
        self.log.get_text()
    }
    pub fn before(lang: &Language) -> Result<(Checks, &Language)> {
        fn select_underlyer(lang: &Language) -> Result<Underlayer> {
            Select::new(
                "Which underlayer tool chain you want to check?",
                Underlayer::options(),
            )
            .with_help_message("current support: Makepad")
            .prompt()
            .map_or_else(
                |_| {
                    Err(Error::Other {
                        ty: Some("select".to_string()),
                        msg: CheckLogs::SelectFailed.t(lang).to_string(),
                    })
                },
                |s| Ok(Underlayer::from_str(s).unwrap()),
            )
        }

        Select::new(&CheckLogs::Select.t(lang).to_string(), Checks::options())
            .prompt()
            .map_or_else(
                |_| {
                    Err(Error::Other {
                        ty: Some("select".to_string()),
                        msg: CheckLogs::SelectFailed.t(lang).to_string(),
                    })
                },
                |check| {
                    let check = Checks::from_str(check).unwrap();
                    match check {
                        Checks::Basic => Ok((check, lang)),
                        Checks::Underlayer(_) => {
                            Ok((Checks::Underlayer(select_underlyer(lang)?), lang))
                        }
                        Checks::All(_) => Ok((Checks::All(select_underlyer(lang)?), lang)),
                    }
                },
            )
    }
    fn handle_running(&mut self, state: CheckState) {
        match state {
            CheckState::Basic => match self.option {
                Checks::Basic => {
                    self.handle_basic();
                    self.log
                        .push(LogItem::info(CheckLogs::Complete.t(&self.lang).to_string()));
                    self.state = ComponentState::Pause;
                }
                Checks::All(_) => {
                    self.handle_basic();
                    self.state.next();
                }
                Checks::Underlayer(_) => {
                    self.state.next();
                }
            },
            CheckState::Underlayer => {
                match self.option {
                    Checks::Underlayer(u) => {
                        self.handle_underlayer(u);
                    }
                    Checks::All(u) => {
                        self.handle_underlayer(u);
                    }
                    Checks::Basic => {}
                }
                self.state.next();
                self.log
                    .push(LogItem::info(CheckLogs::Complete.t(&self.lang).to_string()));
            }
        }
    }
    fn handle_basic(&mut self) {
        let start = std::time::Instant::now();
        let checks = check_basic();
        self.cost.replace(start.elapsed());
        self.log.extend(
            checks
                .iter()
                .map(|item| (item, &self.lang).into())
                .collect::<Vec<LogItem>>(),
        );
        self.items.extend(checks);
    }
    fn handle_underlayer(&mut self, underlayer: Underlayer) {
        let start = std::time::Instant::now();
        let res = service::check::check_underlayer(underlayer);
        match res {
            Ok(checks) => {
                self.cost
                    .replace(self.cost.unwrap_or_default() + start.elapsed());
                self.log.extend(
                    checks
                        .iter()
                        .map(|item| (item, &self.lang).into())
                        .collect::<Vec<LogItem>>(),
                );
                self.items.extend(checks);
            }
            Err(e) => {
                self.log.push(LogItem::error(e.to_string()));
            }
        }
    }
}

impl From<(Checks, &Language)> for CheckCmd {
    fn from(value: (Checks, &Language)) -> Self {
        Self {
            state: Default::default(),
            lang: value.1.clone(),
            option: value.0,
            log: Log::default(),
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
