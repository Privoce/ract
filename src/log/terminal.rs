use std::{borrow::Cow, process::exit};

use colored::{ColoredString, Colorize};
use gen_utils::common::time::local_time_format;

// /// # TerminalLogger
// /// logging messages to the terminal with different colors and styles which can combine i18n with super impl (will be deprecated)
// pub struct TerminalLogger<'a> {
//     pub output: Cow<'a, str>,
// }

// impl<'a> TerminalLogger<'a> {
//     const PREFIX: &'static str = "Ract";
//     fn unified_log(&self, level: ColoredString) -> () {
//         println!(
//             "{} [{}] {} >>> {}",
//             Self::PREFIX.truecolor(255, 112, 67).bold(),
//             local_time_format("%Y-%m-%d %H:%M:%S"),
//             level,
//             self.output
//         )
//     }
//     pub fn info(&self) {
//         self.unified_log("INFO".blue());
//     }
//     pub fn success(&self) {
//         self.unified_log("SUCCESS".green());
//     }
//     /// ## ERROR
//     /// Prints the error message in red and exits the program with a non-zero status code
//     pub fn error(&self) {
//         self.error_no_exit();
//         exit(1);
//     }
//     pub fn error_no_exit(&self) {
//         self.unified_log("ERROR".red().bold());
//     }
//     pub fn warning(&self) {
//         self.unified_log("WARN".yellow().bold());
//     }
//     pub fn new(s: &str) -> TerminalLogger {
//         TerminalLogger {
//             output: Cow::Borrowed(s),
//         }
//     }
//     pub fn logo(&self) {
//         println!("{}", self.output.truecolor(255, 112, 67).bold());
//     }
// }
