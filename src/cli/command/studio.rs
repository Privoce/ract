use crate::{
    app::{AppComponent, ComponentState, Confirm, Dashboard, Select, State},
    common::Result,
    entry::Language,
    log::{Common, Log, LogExt, LogItem, LogType, Options, StudioLogs, UninstallLogs},
    service::{
        check::check_makepad,
        studio::{default_makepad_studio_path, run_gui},
        uninstall::uninstall_all,
    },
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

pub struct StudioCmd {
    lang: Language,
    state: ComponentState<StudioState>,
    log: Log,
    cost: Option<Duration>,
    place: Place,
    selected: bool,
    options: Vec<String>,
}

impl AppComponent for StudioCmd {
    type Output = ();
    type State = StudioState;
    fn new(lang: Language) -> Self {
        let options = vec![
            Common::Option(Options::Default).t(&lang).to_string(),
            Common::Option(Options::Custom).t(&lang).to_string(),
        ];
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            cost: None,
            place: Default::default(),
            selected: true,
            options,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.state {
            ComponentState::Start => {
                self.log.extend(vec![
                    LogItem::info(StudioLogs::Desc.t(&self.lang).to_string()).multi(),
                    LogItem::info(StudioLogs::Check.t(&self.lang).to_string()),
                ]);
                self.state.next();
            }
            ComponentState::Run(state) => {
                self.handle_running(state);
            }
            ComponentState::Pause => {}
            ComponentState::Quit => {}
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up | event::KeyCode::Down => {
                            if self.place.is_select() {
                                self.selected = !self.selected;
                            }
                        }
                        event::KeyCode::Enter => {
                            if self.place.is_select() {
                                self.state.next();
                            } else {
                                self.place = Place::Input;
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
        let (msg, lines) = self.draw_msg(area.width);
        let mut y = 0;
        // here should be 8 - 1 because of the top border is 1
        if lines > 16 {
            y = lines - 16;
        }
        let msg = Paragraph::new(msg)
            .scroll((y, 0))
            .wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP));

        // [dashboard] -----------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Config;
        dashboard.cost = self.cost.clone();
        // [render] -----------------------------------------------------------
        let main_height = if self.state.is_run() {
            match self.place {
                Place::Select => 4,
                Place::Input => 3,
            }
        }else{
            0
        };

        dashboard.render(
            frame,
            area,
            main_height,
            17,
            |frame, [main_area, msg_area]| {
                // [select is use default or custom] --------------------------
                if self.state.is_run() {
                    match self.place {
                        Place::Select => {
                            let selected = if self.selected { 0 } else { 1 };
                            let _ = Select::new_with_options(
                                &StudioLogs::Select.t(&self.lang).to_string(),
                                self.lang,
                                &self.options,
                                Default::default(),
                                None,
                            )
                            .selected(selected)
                            .render_from(main_area, frame);
                        }
                        Place::Input => {}
                    }
                }

                frame.render_widget(msg, msg_area);
            },
        );
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }
}

impl StudioCmd {
    fn draw_msg(&self, w: u16) -> (Text, u16) {
        self.log.draw_text_with_width(w - 2)
    }
    fn handle_running(&mut self, state: StudioState) {
        match state {
            StudioState::Check => {
                // [check makepad env] ------------------------------------------
                let start = Instant::now();
                let res = check_makepad();
                self.cost = Some(start.elapsed());
                match res {
                    Ok(checks) => {
                        let mut err = false;
                        self.log.extend(
                            checks
                                .iter()
                                .map(|item| {
                                    if !item.state {
                                        err = true;
                                    }
                                    (item, &self.lang).into()
                                })
                                .collect::<Vec<LogItem>>(),
                        );
                        if err {
                            self.state.to_pause();
                        } else {
                            self.state.next();
                        }
                    }
                    Err(e) => {
                        self.log.push(LogItem::error(e.to_string()));
                        self.state.to_pause();
                    }
                }
            }
            StudioState::Select => {}
            StudioState::Running => {
                if self.selected {
                    // [use default] ------------------------------------------------
                    let start = Instant::now();
                    match default_makepad_studio_path() {
                        Ok(path) => {
                            self.cost.map(|cost| cost + start.elapsed());
                            match run_gui(path) {
                                Ok(status) => {
                                    if status.success() {
                                        self.log.push(LogItem::error(
                                            StudioLogs::Stop.t(&self.lang).to_string(),
                                        ));
                                    }
                                    self.state.next();
                                }
                                Err(e) => {
                                    self.log.push(LogItem::error(
                                        StudioLogs::Error(e.to_string()).t(&self.lang).to_string(),
                                    ));
                                    self.state.to_pause();
                                }
                            }
                        }
                        Err(e) => {
                            self.log.push(LogItem::error(
                                StudioLogs::Error(e.to_string()).t(&self.lang).to_string(),
                            ));
                            self.state.to_pause();
                        }
                    }
                } else {
                    // [use custom] -----------------------------------------------
                }
            }
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum StudioState {
    #[default]
    Check,
    Select,
    Running,
}

impl State for StudioState {
    fn next(&mut self) -> () {
        match self {
            StudioState::Check => {
                *self = StudioState::Select;
            }
            StudioState::Select => {
                *self = StudioState::Running;
            }
            StudioState::Running => {}
        }
    }

    fn is_run_end(&self) -> bool {
        matches!(self, StudioState::Running)
    }

    fn to_run_end(&mut self) -> () {
        *self = StudioState::Running;
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Place {
    #[default]
    Select,
    Input,
}

impl Place {
    fn next(&mut self) -> () {
        match self {
            Place::Select => {
                *self = Place::Input;
            }
            Place::Input => {}
        }
    }
    fn is_select(&self) -> bool {
        matches!(self, Place::Select)
    }
}
