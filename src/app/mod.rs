use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

pub fn app() -> Result<(), Box<dyn std::error::Error>> {
    let tm = ratatui::init();

    let res = run(tm);
    ratatui::restore();
    res
}

pub fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(draw_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw_app(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}

/// ## Do before app run
/// 1. check update
/// 2. ui init
pub fn before(){
    // [check update] ------------------------------------------------------


    // [ui init] -----------------------------------------------------------

}

pub fn after(){}