use std::borrow::Cow;

use colored::Colorize;

use super::Logs;

pub struct TerminalLogger<'a> {
    pub output: Cow<'a, str>,
}

impl<'a> TerminalLogger<'a> {
    pub fn info(&self) {
        println!("{}", self.output.bright_blue().bold());
    }
    pub fn success(&self) {
        println!("{}", self.output.bright_green().bold());
    }
    pub fn error(&self) {
        println!("{}", self.output.bright_red().bold());
    } 
    pub fn warning(&self) {
        println!("{}", self.output.bright_yellow().bold());
    }
    pub fn rust(&self) {
        println!("{}", self.output.on_truecolor(255, 112, 67).white().bold());
    }
    pub fn new(s: &str) -> TerminalLogger {
        TerminalLogger {
            output: Cow::Borrowed(s),
        }
    }
    pub fn logo(&self) {
        println!("{}", self.output.truecolor(255, 112, 67).bold());
    }
}

