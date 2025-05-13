use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::{str::FromStr, time::Duration};

use crate::{
    app::{self, AppComponent, ComponentState, Dashboard, State},
    entry::{Checks, Language, Underlayer},
    log::{CheckLogs, CommandType, Log, LogExt, LogItem},
    service::{
        self,
        check::{check_basic, CheckItem},
    },
};

pub struct CheckCmd {
    state: ComponentState<CheckState>,
    lang: Language,
    option: Checks,
    selected: usize,
    log: Log,
    items: Vec<CheckItem>,
    cost: Option<Duration>,
}

impl AppComponent for CheckCmd {
    type Output = ();
    type State = CheckState;

    fn new(lang: crate::entry::Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            selected: 0,
            option: Checks::default(),
            log: Log::new(),
            items: vec![],
            cost: None,
        }
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log
                    .push(LogItem::info(CheckLogs::Desc.t(&self.lang).to_string()).multi());
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
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up => {
                            if self.is_running_select() {
                                if self.selected > 0 {
                                    self.selected -= 1;
                                }
                            }
                        }
                        event::KeyCode::Down => {
                            if self.is_running_select() {
                                if self.selected < Checks::options().len() - 1 {
                                    self.selected += 1;
                                }
                            }
                        }
                        event::KeyCode::Enter => {
                            if self.is_running_select() {
                                let options = Checks::options();
                                self.option = Checks::from_str(options[self.selected]).unwrap();
                                self.state.next();
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
        let (msg, lines) = self.draw_msg(area.width - 4);
        let mut y = 0;
        if lines > 10 {
            y = lines - 10;
        }
        let msg = Paragraph::new(msg)
            .scroll((y, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));
        // [dashboard] ----------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = CommandType::Check;
        dashboard.cost = self.cost;
        // [render components] ------------------------------------------------------------------------------------
        if self.is_running_select() {
            dashboard.render(frame, area, 5, 12, |frame, [main_area, msg_area]| {
                let options = Checks::options();
                let _ = app::Select::new_with_options(
                    &CheckLogs::Select.t(&self.lang).to_string(),
                    self.lang,
                    &options,
                    Color::White.into(),
                    None,
                )
                .selected(self.selected)
                .render_from(main_area, frame);
                frame.render_widget(msg, msg_area);
            });
        } else {
            let len = self.items.len();
            // [render items] ----------------------------------------------------------------------------------------------
            let (items, list_height): (Vec<ListItem>, u16) = self.items.iter().enumerate().fold(
                (vec![], 0),
                |(mut items, mut height), (index, item)| {
                    let item = item.draw_list(len == index + 1);
                    height += item.height() as u16;
                    items.push(item);

                    (items, height)
                },
            );
            let list = List::new(items);
            dashboard.render(
                frame,
                area,
                list_height,
                12,
                |frame, [main_area, msg_area]| {
                    frame.render_widget(list, main_area);
                    frame.render_widget(msg, msg_area);
                },
            );
        }
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl CheckCmd {
    fn draw_msg(&self, w: u16) -> (Text, u16) {
        self.log.draw_text_with_width(w)
    }
    fn is_running_select(&self) -> bool {
        matches!(self.state, ComponentState::Run(CheckState::Select))
    }
    fn handle_running(&mut self, state: CheckState) {
        match state {
            CheckState::Select => {}
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

#[derive(Default, Clone, Copy, Debug)]
pub enum CheckState {
    #[default]
    Select,
    Basic,
    Underlayer,
}

impl State for CheckState {
    fn next(&mut self) -> () {
        match self {
            CheckState::Select => {
                *self = CheckState::Basic;
            }
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
