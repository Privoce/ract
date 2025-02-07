use std::{
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    str::FromStr,
};

use gen_utils::{common::stream_terminal, error::Error};
use inquire::{Confirm, Text};

use crate::core::{
    command::check::current_states,
    entry::ChainEnvToml,
    log::{InstallLogs, StudioLogs, TerminalLogger},
};

pub fn run() -> () {
    StudioLogs::Welcome.terminal().rust();
    StudioLogs::Desc.terminal().info();

    if let Err(e) = conf_run() {
        TerminalLogger::new(e.to_string().as_str()).error();
        exit(2);
    }
}

/// run makepad studio
/// now support gui platform
fn conf_run() -> Result<(), Error> {
    let states = current_states()?;

    if !states.underlayer.makepad_is_ok() {
        return Err(InstallLogs::UnInstalled("makepad".to_string())
            .to_string()
            .into());
    }

    let is_default = Confirm::new("Do you want to run default studio?")
        .with_default(true)
        .prompt()
        .map_err(|e| e.to_string())?;

    let path = if is_default {
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
    } else {
        let path = Text::new("Path for the target studio")
            .prompt()
            .map_err(|e| e.to_string())?;

        let path = PathBuf::from_str(&path).map_err(|e| e.to_string())?;
        if !path.exists() {
            Err(Error::from("The path is not exist!"))
        } else {
            Ok(path)
        }
    }?;

    run_gui(path)
}

fn run_gui<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    StudioLogs::Gui.terminal().info();
    // cargo run -p makepad-studio --release
    let mut child = Command::new("cargo")
        .args(&["run", "-p", "makepad-studio", "--release"])
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
                StudioLogs::Stop.terminal().success();
                Ok(())
            } else {
                Err(StudioLogs::Error.to_string().into())
            }
        },
    )
}
