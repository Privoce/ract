use std::{
    path::Path,
    process::{Command, Stdio},
};

use gen_utils::{common::stream_terminal, error::Error};

use crate::{entry::Language, log::{LogExt, LogItem, ProjectLogs}};

pub fn run<P>(path: P, lang: Language) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    ProjectLogs::Start.info(lang).print();
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
        |line| LogItem::info(line).print(),
        |line| LogItem::warning(line).print(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                ProjectLogs::Stop.success(lang).print();
                Ok(())
            } else {
                Err(ProjectLogs::Error("-".to_string()).to_string().into())
            }
        },
    )
}
