use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

use gen_utils::error::Error;

/// # Run Makepad wasm
/// which is async and will return a child process if success
pub fn run<P>(path: P, project: &str, port: u16) -> Result<Child, Error>
where
    P: AsRef<Path>,
{
    // cargo makepad wasm --port=8010 run -p ${project} --release
    let child = Command::new("cargo")
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

    // std::thread::spawn(move || stream_terminal(&mut child, info, err));

    // WasmLogs::Package.terminal(lang).info();
    // WasmLogs::Start.terminal(lang).info();
    // stream_terminal(
    //     &mut child,
    //     |line| TerminalLogger::new(&line).info(),
    //     |line| TerminalLogger::new(&line).warning(),
    // )
    // .map_or_else(
    //     |e| Err(e),
    //     |status| {
    //         if status.success() {
    //             WasmLogs::Stop.terminal(lang).success();
    //             Ok(())
    //         } else {
    //             Err(ProjectLogs::Error("-".to_string()).to_string().into())
    //         }
    //     },
    // )
    Ok(child)
}
