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

/// # Uninstall Ract
/// 由于Ract是一个工具链，所以卸载Ract就是删除Ract的所有文件，包括:
/// 1. Ract的环境配置文件(.env)
/// 2. .env同级的chain目录(chain是链依赖目录，存放链依赖的环墫配置文件和相关的依赖包)
/// 3. Ract的可执行文件(name = ract)
pub struct UninstallCmd {
    lang: Language,
    state: ComponentState<BaseRunState>,
    log: Log,
    selected: bool,
}

impl AppComponent for UninstallCmd {
    type Output = ();
    type State = BaseRunState;

    fn new(lang: Language) -> Self {
        Self {
            lang,
            state: Default::default(),
            log: Log::new(),
            selected: false,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.quit(),
                        event::KeyCode::Up | event::KeyCode::Down => {
                            // [use up and down to change weather to uninstall Ract] -----------------------------------------
                            self.selected = !self.selected;
                        }
                        event::KeyCode::Enter => {
                            // [if selected is true, run uninstall service] --------------------------------------------------
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
    /// ## Render the uninstall Confirm Component
    /// this component will ask user to confirm uninstall Ract
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

    fn state(&self) -> &ComponentState<Self::State> {
        &self.state
    }

    fn quit(&mut self) -> () {
        self.state.quit();
    }
}
