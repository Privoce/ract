use colored::Colorize;

pub struct TerminalLogger {
    pub output: String,
}

impl TerminalLogger {
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
            output: s.to_string(),
        }
    }
    pub fn logo(&self) {
        println!("{}", self.output.truecolor(255, 112, 67).bold());
    }
}
