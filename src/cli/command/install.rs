use crate::{
    app::{
        AppComponent, BaseRunState, ComponentState, Dashboard, MultiSelect, State, Timeline,
        TimelineState,
    },
    entry::{Language, Tools},
    log::{InstallLogs, Log, LogExt, LogItem, LogType},
    service::check::{
        check_basic, check_cargo, check_git, check_rustc, check_underlayer, CheckItem,
    },
};
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

pub struct InstallCmd {
    lang: Language,
    state: ComponentState<InstallState>,
    log: Log,
    cost: Option<Duration>,
    check: Check,
    selecteds: Vec<usize>,
    selected: usize,
}

impl AppComponent for InstallCmd {
    type Output = ();

    type State = InstallState;

    fn new(lang: Language) -> Self {
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            cost: None,
            check: Check::default(),
            selecteds: vec![0],
            selected: 0,
        }
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log
                    .push(LogItem::info(InstallLogs::Desc.t(&self.lang).to_string()).multi());
                self.state.next();
            }
            ComponentState::Run(state) => match state {
                InstallState::Check(state) => {
                    let err = false;
                    self.handle_check(err, state);

                    if err {
                        self.check.state = TimelineState::Failed;
                        self.state.to_pause();
                    } else {
                        if state.is_run_end() {
                            self.check.state = TimelineState::Success;
                        }
                    }
                    self.state.next();
                }
                InstallState::Select => {}
                InstallState::Install => {}
            },
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up => {
                            if self.selected > 0 {
                                self.selected -= 1;
                            }
                        }
                        event::KeyCode::Down => {
                            if self.selected < Tools::options().len() - 1 {
                                self.selected += 1;
                            }
                        }
                        event::KeyCode::Char(' ') => {
                            if self.selecteds.contains(&self.selected) {
                                self.selecteds.retain(|&x| x != self.selected);
                            } else {
                                self.selecteds.push(self.selected);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let (msg, lines) = self.log.draw_text_with_width(area.width - 2);
        let mut y = 0;
        if lines > 12 {
            y = lines - 12;
        }
        let msg = Paragraph::new(msg)
            .scroll((y as u16, 0))
            .wrap(Wrap { trim: true });
        // [multi check] ------------------------------------------------------------
        let timeline = Timeline::new(InstallLogs::CheckTitle.t(&self.lang).to_string(), self.lang)
            .progress(self.check.progress)
            .cost(self.cost.unwrap_or_default())
            .description(self.check.to_log().t(&self.lang).to_string())
            .state(self.check.state)
            .draw();
        // [dashboard] -----------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Install;
        dashboard.cost = self.cost.clone();

        // [multi select] --------------------------------------------------------
        let (main_height, mut multi_select, layout) =
            if let ComponentState::Run(run_state) = self.state {
                match run_state {
                    InstallState::Check(_) => (
                        timeline.height,
                        None,
                        Layout::vertical([Constraint::Percentage(100)]),
                    ),
                    InstallState::Select => {
                        let multi_select = MultiSelect::new(
                            InstallLogs::Select.t(&self.lang).to_string(),
                            self.lang,
                            &Tools::options(),
                            Default::default(),
                            None,
                        )
                        .selecteds(self.selecteds.clone())
                        .selected(self.selected);
                    
                        let multi_select_height = multi_select.height(area.width - 4);
                        (
                            timeline.height + multi_select_height,
                            Some(multi_select),
                            Layout::vertical([
                                Constraint::Length(timeline.height),
                                Constraint::Length(multi_select_height),
                            ]),
                        )
                    }
                    InstallState::Install => (
                        timeline.height,
                        None,
                        Layout::vertical([Constraint::Percentage(100)]),
                    ),
                }
            } else {
                (
                    timeline.height,
                    None,
                    Layout::vertical([Constraint::Percentage(100)]),
                )
            };

        // let path = "/Users/shengyifei/projects/gen_ui/ract_workspace/ract/log";
        // gen_utils::common::fs::write(path, & format!("{}|{}|{}", main_height, timeline.height, multi_select_height));

        dashboard.render(
            frame,
            area,
            main_height,
            13,
            |frame, [main_area, msg_area]| {
                // [layout] -----------------------------------------------------------------
                let multi_check_area = if let Some(multi_select) = multi_select.as_mut() {
                    let [multi_check_area, select_area] = layout
                        .flex(ratatui::layout::Flex::SpaceBetween)
                        .areas(main_area);
                    multi_select.render_from(select_area, frame);
                    multi_check_area
                } else {
                    let [multi_check_area] = layout
                        .flex(ratatui::layout::Flex::SpaceBetween)
                        .areas(main_area);
                    multi_check_area
                };

                timeline.render(multi_check_area, frame);
                // [install progress] -------------------------------------------------------
                frame.render_widget(msg, msg_area);
            },
        );
    }

    fn state(&self) -> &ComponentState<Self::State>
    where
        Self::State: crate::app::State,
    {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl InstallCmd {
    fn handle_check(&mut self, mut err: bool, state: CheckState) -> () {
        let start = Instant::now();
        // [check basic env] ------------------------------------------------
        let check_res = match state {
            CheckState::Rustc => Ok(vec![check_rustc()]),
            CheckState::Cargo => Ok(vec![check_cargo()]),
            CheckState::Git => Ok(vec![check_git()]),
            CheckState::Underlayer => check_underlayer(crate::entry::Underlayer::Makepad),
        };

        if let Some(cost) = self.cost.as_mut() {
            *cost += start.elapsed();
        } else {
            self.cost = Some(start.elapsed());
        }

        match check_res {
            Ok(res) => {
                self.log.extend(
                    res.iter()
                        .map(|item| {
                            if !item.state {
                                err = true;
                            } else {
                                self.check.items.push(item.clone());
                                self.check.progress += 20;
                            }
                            (item, &self.lang).into()
                        })
                        .collect::<Vec<LogItem>>(),
                );
            }
            Err(e) => {
                self.log.push(LogItem::error(e.to_string()));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InstallState {
    Check(CheckState),
    Select,
    Install,
}

impl Default for InstallState {
    fn default() -> Self {
        Self::Check(CheckState::Rustc)
    }
}

impl State for InstallState {
    fn next(&mut self) -> () {
        match self {
            InstallState::Check(state) => {
                if state.is_run_end() {
                    *self = InstallState::Select;
                } else {
                    state.next();
                }
            }
            InstallState::Select => {
                *self = InstallState::Install;
            }
            InstallState::Install => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, InstallState::Install)
    }

    fn to_run_end(&mut self) -> () {
        *self = InstallState::Install;
    }
}

#[derive(Default, Clone, Debug, Copy)]
pub enum CheckState {
    #[default]
    Rustc,
    Cargo,
    Git,
    Underlayer,
}

impl State for CheckState {
    fn next(&mut self) -> () {
        match self {
            CheckState::Rustc => {
                *self = CheckState::Cargo;
            }
            CheckState::Cargo => {
                *self = CheckState::Git;
            }
            CheckState::Git => *self = CheckState::Underlayer,
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

#[derive(Default, Clone, Debug)]
struct Check {
    pub progress: u16,
    pub items: Vec<CheckItem>,
    pub state: TimelineState,
}

impl Check {
    pub fn to_log(&self) -> InstallLogs {
        let current = self
            .items
            .last()
            .unwrap_or(&CheckItem {
                name: "-".to_string(),
                path: None,
                state: false,
            })
            .name
            .to_string();
        let num = self.items.len();
        InstallLogs::Check {
            current,
            num: num as u8,
            total: 5,
        }
    }
}
