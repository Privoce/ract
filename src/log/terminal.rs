use std::{borrow::Cow, process::exit};

use colored::{ColoredString, Colorize};

/// # TerminalLogger
/// logging messages to the terminal with different colors and styles which can combine i18n with super impl
pub struct TerminalLogger<'a> {
    pub output: Cow<'a, str>,
}

impl<'a> TerminalLogger<'a> {
    const PREFIX: &'static str = "Ract :: ";
    fn unified_log(&self, level: ColoredString) -> () {
        println!(
            "{}{} >>> {}",
            Self::PREFIX.on_truecolor(255, 112, 67).bold(),
            level,
            self.output
        )
    }

    pub fn info(&self) {
        self.unified_log("INFO".blue());
    }
    pub fn success(&self) {
        self.unified_log("SUCCESS".green());
    }
    /// ## ERROR
    /// Prints the error message in red and exits the program with a non-zero status code
    pub fn error(&self) {
        self.unified_log("ERROR".bright_red().bold());
        exit(1);
    }
    pub fn warning(&self) {
        self.unified_log("WARNING".bright_yellow().bold());
    }
    // pub fn rust(&self) {
    //     println!("{}", self.output.on_truecolor(255, 112, 67).white().bold());
    // }
    pub fn new(s: &str) -> TerminalLogger {
        TerminalLogger {
            output: Cow::Borrowed(s),
        }
    }
    pub fn logo(&self) {
        println!("{}", self.output.truecolor(255, 112, 67).bold());
    }
}
