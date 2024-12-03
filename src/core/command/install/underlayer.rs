use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
    str::FromStr,
};

use gen_utils::error::Error;
use inquire::{Confirm, MultiSelect};

use crate::core::{
    entry::Tools,
    env::{gen_components_path, makepad_widgets_path, real_chain_env_toml},
    log::{InstallLogs, TerminalLogger},
};

pub fn install_makepad(tools: &Tools) -> Result<(), Error> {
    let makepad_is_ok = tools.underlayer.makepad_is_ok();
    let content = real_chain_env_toml()?;

    let mut makepad_path = PathBuf::from_str(
        content["dependencies"]["makepad-widgets"]
            .as_str()
            .unwrap_or(makepad_widgets_path().to_str().unwrap()),
    )
    .unwrap();
    makepad_path.pop();

    let mut gen_components_path = PathBuf::from_str(
        content["dependencies"]["gen_components"]
            .as_str()
            .unwrap_or(gen_components_path().to_str().unwrap()),
    )
    .unwrap();

    gen_components_path.pop();

    MultiSelect::new(
        "What you need to (re)install?",
        vec![
            "gen_components",
            "android_build",
            "ios_build",
            "wasm_build",
            "makepad_studio",
            "default",
        ],
    )
    .with_help_message("if you do not want to install other tools just makepad, select default!")
    .prompt()
    .map_or_else(
        |e| Err(e.to_string().into()),
        |options| {
            // first you must install makepad (so check and ask user need to update?)
            update_makepad(makepad_is_ok, makepad_path.as_path())?;
            // then you can install other tools
            for option in options {
                let res = match option {
                    "gen_components" => clone_gen_ui_components(gen_components_path.clone()),
                    "android_build" => install_android_build(makepad_path.as_path()),
                    "ios_build" => install_ios_build(makepad_path.as_path()),
                    "wasm_build" => install_wasm_build(makepad_path.as_path()),
                    "makepad_studio" => install_makepad_studio(makepad_path.as_path()),
                    "default" => Ok(()),
                    _ => Err("❌ Invalid option!".to_string().into()),
                };

                if res.is_err() {
                    return res;
                }
            }
            Ok(())
        },
    )
}

fn install_makepad_studio<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo install makepad-studio
    if check_cargo_makepad(path.as_ref()).unwrap_or_default() {
        return Command::new("cargo")
            .args(&["install", "makepad-studio"])
            .current_dir(path)
            .output()
            .map_or_else(
                |e| Err(e.to_string().into()),
                |out| {
                    if out.status.success() {
                        InstallLogs::Confirm("makepad_studio".to_string())
                            .terminal()
                            .success();
                        TerminalLogger::new("ℹ️ You can cd to makepad dir and use `cargo run -p makepad-studio --release` to open the makepad studio. Or you can use `ract run` to open the makepad studio")
                            .info();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("makepad_studio".to_string())
                            .to_string()
                            .into())
                    }
                },
            );
    } else {
        TerminalLogger::new("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad").error();
        exit(2);
    }
}

fn install_android_build<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo makepad android install-toolchain
    if check_cargo_makepad(path.as_ref()).unwrap_or_default() {
        return Command::new("cargo")
            .args(&["makepad", "android", "install-toolchain"])
            .current_dir(path)
            .output()
            .map_or_else(
                |e| Err(e.to_string().into()),
                |out| {
                    if out.status.success() {
                        InstallLogs::Confirm("android_build".to_string())
                            .terminal()
                            .success();
                        TerminalLogger::new("ℹ️ You can use `cargo makepad android run -p ${project_name} --release` to run the project")
                            .info();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("android_build".to_string())
                            .to_string()
                            .into())
                    }
                },
            );
    } else {
        // means cargo makepad is not installed or has some error, return Err
        TerminalLogger::new("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad").error();
        exit(2);
    }
}

fn install_ios_build<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // xcode-select --install
    // cargo makepad apple ios install-toolchain
    if check_xcode_select().unwrap_or_default() {
        if check_cargo_makepad(path.as_ref()).unwrap_or_default() {
            return Command::new("cargo")
            .args(&["makepad", "apple", "ios", "install-toolchain"])
            .current_dir(path)
            .output()
            .map_or_else(
                |e| Err(e.to_string().into()),
                |out| {
                    if out.status.success() {
                        InstallLogs::Confirm("ios_build".to_string())
                            .terminal()
                            .success();
                        TerminalLogger::new("ℹ️ You can use `cargo makepad apple ios --org=my.test --app=${project_name} run-sim -p ${project_name} --release` to run the project\nFor more information, see: https://github.com/syf20020816/makepad/tree/rik?tab=readme-ov-file#5-ios-setup--install")
                            .info();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("ios_build".to_string())
                            .to_string()
                            .into())
                    }
                },
            );
        } else {
            TerminalLogger::new("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad").error();
            exit(2);
        }
    } else {
        TerminalLogger::new("❗️ xcode-select is not installed, please install it first!").error();
        exit(2);
    }
}

fn check_xcode_select() -> Result<bool, Error> {
    Command::new("xcode-select")
        .args(&["--install"])
        .output()
        .map_or_else(
            |_| {
                InstallLogs::UnInstalled("xcode-select".to_string())
                    .terminal()
                    .warning();
                Ok(false)
            },
            |out| {
                if out.status.success() {
                    InstallLogs::Installed("xcode-select".to_string())
                        .terminal()
                        .success();
                    Ok(true)
                } else {
                    InstallLogs::UnInstalled("xcode-select".to_string())
                        .terminal()
                        .warning();
                    Ok(false)
                }
            },
        )
}

fn install_wasm_build<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo makepad wasm install-toolchain
    if check_cargo_makepad(path.as_ref()).unwrap_or_default() {
        return Command::new("cargo")
            .args(&["makepad", "wasm", "install-toolchain"])
            .current_dir(path)
            .output()
            .map_or_else(
                |e| Err(e.to_string().into()),
                |out| {
                    if out.status.success() {
                        InstallLogs::Confirm("wasm_build".to_string())
                            .terminal()
                            .success();
                        TerminalLogger::new("ℹ️ You can use `cargo makepad wasm run -p ${project_name} --release` to run the project")
                            .info();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("wasm_build".to_string())
                            .to_string()
                            .into())
                    }
                },
            );
    } else {
        TerminalLogger::new("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad").error();
        exit(2);
    }
}

fn check_cargo_makepad<P>(path: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    // use cargo makepad -h
    Command::new("cargo")
        .args(&["makepad", "-h"])
        .current_dir(path)
        .output()
        .map_or_else(
            |_| {
                InstallLogs::UnInstalled("cargo_makepad".to_string())
                    .terminal()
                    .warning();
                Ok(false)
            },
            |out| {
                if out.status.success() {
                    InstallLogs::Installed("cargo_makepad".to_string())
                        .terminal()
                        .success();
                    Ok(true)
                } else {
                    InstallLogs::UnInstalled("cargo_makepad".to_string())
                        .terminal()
                        .warning();
                    Ok(false)
                }
            },
        )
}

fn install_cargo_makepad<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo install --path ./tools/cargo_makepad
    InstallLogs::Install("cargo_makepad".to_string())
        .terminal()
        .info();
    Command::new("cargo")
        .args(&["install", "--path", "./tools/cargo_makepad"])
        .current_dir(path)
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Confirm("cargo_makepad".to_string())
                        .terminal()
                        .success();
                    TerminalLogger::new(
                        "ℹ️ You can use `cargo makepad -h` to see the help information",
                    )
                    .info();
                    Ok(())
                } else {
                    Err(InstallLogs::InstallErr("cargo_makepad".to_string())
                        .to_string()
                        .into())
                }
            },
        )
}

fn update_makepad<P>(install: bool, path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // check makepad
    if install {
        // if makepad is ok, ask user to update
        if check_cargo_makepad(path.as_ref()).unwrap_or_default() {
            return Confirm::new("Do you want to update makepad?")
                .with_default(false)
                .prompt()
                .map_or_else(
                    |e| Err(e.to_string().into()),
                    |res| {
                        if res {
                            clone_makepad(path.as_ref())?;
                            install_cargo_makepad(path)
                        } else {
                            Ok(())
                        }
                    },
                );
        } else {
            TerminalLogger::new("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad").error();
            exit(2);
        }
    } else {
        TerminalLogger::new("❗️ Makepad is not installed, now installing Makepad").warning();
        clone_makepad(path.as_ref())?;
        install_cargo_makepad(path)
    }
}

fn clone_makepad<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    InstallLogs::Install("makepad".to_string())
        .terminal()
        .info();
    // use git clone makepad: git clone --branch rik --depth 1 https://github.com/syf20020816/makepad.git makepad
    Command::new("git")
        .args(&[
            "clone",
            "--branch",
            "rik",
            "--depth",
            "1",
            "https://github.com/syf20020816/makepad.git",
            "makepad",
        ])
        .current_dir(path)
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Confirm("makepad".to_string())
                        .terminal()
                        .success();
                    Ok(())
                } else {
                    Err("❌ makepad install failed".to_string().into())
                }
            },
        )
}

fn clone_gen_ui_components<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    InstallLogs::Install("gen_components".to_string())
        .terminal()
        .info();
    // https://github.com/Privoce/GenUI-Builtin-Component.git gen_components
    Command::new("git")
        .args(&[
            "clone",
            "--branch",
            "main",
            "--depth",
            "1",
            "https://github.com/Privoce/GenUI-Builtin-Component.git",
            "gen_components",
        ])
        .current_dir(path)
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Confirm("gen_components".to_string())
                        .terminal()
                        .success();
                    Ok(())
                } else {
                    Err("❌ gen_components install failed".to_string().into())
                }
            },
        )
}
