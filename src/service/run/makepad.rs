use std::{
    path::Path,
    process::{Command, Stdio},
};

use gen_utils::{common::stream_terminal, error::Error};

use crate::log::{ProjectLogs, TerminalLogger};

pub fn run<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    ProjectLogs::Start.terminal().info();
    // run: cargo run
    let mut child = Command::new("cargo")
        .args(&["run"])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    stream_terminal(
        &mut child,
        |line| TerminalLogger::new(&line).info(),
        |line| TerminalLogger::new(&line).warning(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                ProjectLogs::Stop.terminal().success();
                Ok(())
            } else {
                Err(ProjectLogs::Error("-".to_string()).to_string().into())
            }
        },
    )
}
