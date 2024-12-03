use std::{
    path::Path,
    process::{Command, Stdio},
};

use gen_utils::{common::stream_terminal, error::Error};

use crate::core::log::{ProjectLogs, TerminalLogger, WasmLogs};

pub fn run<P>(path: P, project: &str, port: u16) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo makepad wasm --port=8010 run -p ${project} --release
    let mut child = Command::new("cargo")
        .args(&[
            "makepad",
            "wasm",
            &format!("--port={}", port),
            "run",
            "-p",
            project,
            "--release",
        ])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    
    WasmLogs::Package.terminal().info();
    WasmLogs::Start.terminal().info();
    stream_terminal(
        &mut child,
        |line| TerminalLogger::new(&line).info(),
        |line| TerminalLogger::new(&line).warning(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                WasmLogs::Stop.terminal().success();
                Ok(())
            } else {
                Err(ProjectLogs::Error("-".to_string()).to_string().into())
            }
        },
    )
}
