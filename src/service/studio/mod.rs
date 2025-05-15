use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use gen_utils::{common::stream_terminal, error::Error};

use crate::entry::ChainEnvToml;

pub fn default_makepad_studio_path() -> Result<PathBuf, Error> {
    let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let makepad_studio_path = chain_env_toml
        .makepad_widgets_path()
        .map_or_else(
            || {
                Err(Error::from(
                    "can not find [dependencies.makepad-widgets] in env.toml, maybe config broken, use `ract init` to fix it",
                ))
            },
            |path| Ok(path.join("studio")),
        )?;

    if !makepad_studio_path.exists() {
        Err(Error::from("The path is not exist!"))
    } else {
        Ok(makepad_studio_path)
    }
}

pub fn run_gui<P, I, E>(path: P, info: I, err: E) -> Result<ExitStatus, Error>
where
    P: AsRef<Path>,
    I: Fn(String) + Send + 'static,
    E: Fn(String) + Send + 'static,
{
    // StudioLogs::Gui.terminal().info();
    // cargo run -p makepad-studio --release
    let mut child = Command::new("cargo")
        .args(&["run", "-p", "makepad-studio", "--release"])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    stream_terminal(&mut child, info, err)
}
