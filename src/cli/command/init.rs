use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Gauge, List, ListItem, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    app::{AppComponent, Dashboard, TimelineItem, TimelineState},
    cli::command,
    entry::Language,
    log::{InitLogs, LogItem, LogType},
};

pub struct InitCmd {
    state: InitState,
    lang: Language,
    progress: f64,
    logs: Vec<LogItem>,
}

// pub fn run(lang: &Language) -> crate::common::Result<()>{

// }

impl AppComponent for InitCmd {
    fn new(lang: Language) -> Self {
        Self {
            state: Default::default(),
            lang,
            progress: 0.0,
            logs: vec![],
        }
    }
    fn run(mut self, terminal: &mut DefaultTerminal) -> crate::common::Result<()> {
        while !self.state.is_quit() {
            // terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            // self.update(terminal.size()?.width);
        }

        Ok(())
    }

    fn handle_events(&mut self) -> crate::common::Result<()> {
        let mut do_next = false;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Enter => {
                            do_next = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if do_next {
            // handle service
            match self.state {
                InitState::Start => {
                    self.logs.push(LogItem::info(InitLogs::Init.to_string()));
                    self.state.next();
                }
                InitState::Run(run_state) => match run_state {
                    RunState::CreateEnvFile => {}
                    RunState::CreateChain => {}
                },
                InitState::Quit => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}

impl InitCmd {
    /// ## Render the init command
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let msg = self.render_msg();

        // [dashboard] -------------------------------------------------------------------------------------------
        let mut dashboard = Dashboard::new(self.lang.clone());
        dashboard.ty = LogType::Init;
        
        // [render app] ------------------------------------------------------------------------------------------

        // dashboard.render(frame, dashboard_area, |frame, area| {
        //     self.render_dashboard(&dashboard, frame, area)
        // });

        let mut node1 = TimelineItem::new("Test1", self.lang)
            .description("Test1 description")
            .render();

       
        // let (header_left, header_right) = node1.header;

        let mut node2 = TimelineItem::new("Test2", self.lang).render();

        // node1.render(area1, frame);
        // node2.render(area2, frame);

        let container_height = node1.height + node2.height + 1 + 2;

        let layout = Layout::vertical([
            Constraint::Length(msg.height() as u16),
            Constraint::Length(container_height),
        ])
        .spacing(1)
        .vertical_margin(1);
        let [msg_area, dashboard_area] = layout.areas(area);
        
        

        // [render components] -------------------------------------------------------
        frame.render_widget(msg, msg_area);
        // dashboard.render_container(frame, dashboard_area);
        // frame.render_widget(dasah, area);
        dashboard.render(frame, dashboard_area, |frame, area| {
            let [node1_area, node2_area] = Layout::vertical([
                Constraint::Length(node1.height),
                Constraint::Length(node2.height),
            ])
            .spacing(1)
            .areas(area);
        let node1_container = Block::new();
        // let node1_inner_area = node1_container.inner(node1_area);
            let [header_area, main_area, footer_area] = node1.layout.areas(node1_area);
            let [header_left_area, header_right_area] = Layout::horizontal([
                Constraint::Length(node1.header.0.width() as u16),
                Constraint::Length(node1.header.1.width() as u16),
            ])
            .spacing(1)
            .areas(header_area);
           

            let header = Block::new();
            // frame.render_widget(node1_container, node1_area);
            frame.render_widget(header, header_area);
            frame.render_widget(node1.header.0, header_left_area);
            frame.render_widget(node1.header.1, header_right_area);
            // frame.render_widget(node1.main.unwrap(), main_area);
            // frame.render_widget(node1.footer.0, footer_area);

        });
       
    }

    fn render_msg(&self) -> Text {
        let items: Vec<Line> = self.logs.iter().map(|log| log.fmt_line()).collect();
        Text::from_iter(items)
    }

    fn draw_components(&self) {
        let mut node1 = TimelineItem::new("Test1", self.lang);
        node1.description.replace("Test1 description".to_string());
        node1.render();
        let mut node2 = TimelineItem::new("Test2", self.lang);
        node2.render();
    }

    // fn render_dashboard(
    //     &self,
    //     dashboard: &Dashboard,
    //     frame: &mut Frame,
    //     area: ratatui::prelude::Rect,
    // ) {
    //     // let [area1, area2] =
    //     //     Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
    //     //         .spacing(1)
    //     //         .areas(area);

    //     let mut node1 = TimelineItem::new("Test1", self.lang);
    //     node1.description.replace("Test1 description".to_string());
    //     node1.render();

    //     let header = Block::new();
    //     let (header_left, header_right) = node1.header;
    //     let [header_left_area, header_right_area] = Layout::horizontal([
    //         Constraint::Length(header_left.width() as u16),
    //         Constraint::Length(header_right.width() as u16),
    //     ])
    //     .spacing(1)
    //     .areas(header_area);

    //     let [header_area, main_area, footer_area] = node1.layout.areas()

    //     let mut node2 = TimelineItem::new("Test2", self.lang);
    //     node2.render();

    //     // node1.render(area1, frame);
    //     // node2.render(area2, frame);

    //     Layout::vertical([
    //         Constraint::Length(node1.height),
    //         Constraint::Length(node2.height),
    //     ]);

    //     let container_height = node1.height + node2.height;

    // }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum InitState {
    #[default]
    Start,
    Run(RunState),
    Quit,
}

impl InitState {
    pub fn quit(&mut self) {
        *self = InitState::Quit;
    }
    pub fn is_quit(&self) -> bool {
        matches!(self, InitState::Quit)
    }
    pub fn is_start(&self) -> bool {
        matches!(self, InitState::Start)
    }
    pub fn next(&mut self) {
        match self {
            InitState::Start => {
                *self = InitState::Run(RunState::default());
            }
            InitState::Run(run_state) => match run_state {
                RunState::CreateEnvFile => {
                    *self = InitState::Run(RunState::CreateChain);
                }
                RunState::CreateChain => {
                    *self = InitState::Quit;
                }
            },
            InitState::Quit => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum RunState {
    #[default]
    CreateEnvFile,
    CreateChain,
}
