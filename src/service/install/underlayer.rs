use std::{
    path::Path,
    process::{exit, Command, Stdio},
};

use gen_utils::{
    common::{fs, stream_terminal},
    error::Error,
};
use inquire::Confirm;
use which::which;

use crate::{
    entry::MakepadTools,
    log::{InstallLogs, TerminalLogger},
};

pub fn install<P>(path: P, makepad_is_ok: bool, makepad_tools: MakepadTools) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    update_makepad(makepad_is_ok, path.as_ref())?;

    match makepad_tools {
        crate::entry::MakepadTools::Makepad => {}
        crate::entry::MakepadTools::GenUi => {
            clone_gen_ui_components(path)?;
        }
        crate::entry::MakepadTools::Android => {
            install_android_build(path)?;
        }
        crate::entry::MakepadTools::Ios => {
            install_ios_build(path)?;
        }
        crate::entry::MakepadTools::Wasm => {
            install_wasm_build(path)?;
        }
        crate::entry::MakepadTools::Studio => {
            install_makepad_studio(path)?;
        }
    }

    Ok(())
}

fn install_makepad_studio<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo install makepad-studio
    if check_cargo_makepad() {
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
                        TerminalLogger::new("ℹ You can cd to makepad dir and use `cargo run -p makepad-studio --release` to open the makepad studio. Or you can use `ract run` to open the makepad studio")
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
    if check_cargo_makepad() {
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
                        TerminalLogger::new("ℹ You can use `cargo makepad android run -p ${project_name} --release` to run the project")
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
    if which("xcode-select").is_ok() {
        // 如果已安装，可以继续判断是否设置了开发者目录
        // 检查是否设置了开发者目录
        let xcode_path_check = Command::new("xcode-select")
            .args(&["-p"])
            .output()
            .map_or(false, |out| out.status.success());

        if !xcode_path_check {
            TerminalLogger::new("❗️ Xcode command line tools are installed but not properly configured. Please run 'xcode-select --install' to complete setup.").warning();
            exit(2);
        }

        if check_cargo_makepad() {
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
                        TerminalLogger::new("ℹ You can use `cargo makepad apple ios --org=my.test --app=${project_name} run-sim -p ${project_name} --release` to run the project\nFor more information, see: https://github.com/syf20020816/makepad/tree/rik?tab=readme-ov-file#5-ios-setup--install")
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

fn install_wasm_build<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo makepad wasm install-toolchain
    if check_cargo_makepad() {
        let mut child = Command::new("cargo")
            .args(&["makepad", "wasm", "install-toolchain"])
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        return stream_terminal(
            &mut child,
            |line| TerminalLogger::new(&line).info(),
            |line| TerminalLogger::new(&line).warning(),
        ).map_or_else(
            |e| Err(e),
            |status| {
                if status.success() {
                    InstallLogs::Confirm("wasm_build".to_string())
                    .terminal()
                    .success();
                TerminalLogger::new("ℹ You can use `cargo makepad wasm run -p ${project_name} --release` to run the project")
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
        return Err("❗️ cargo makepad is not installed, please install it first! If you get this error, do update makepad".to_string().into());
    }
}

fn check_cargo_makepad() -> bool {
    which("cargo-makepad").is_ok()
}

fn install_cargo_makepad<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo install --path ./tools/cargo_makepad
    InstallLogs::Install("cargo_makepad".to_string())
        .terminal()
        .info();
    let mut child = Command::new("cargo")
        .args(&["install", "--path", "./makepad/tools/cargo_makepad"])
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
                InstallLogs::Confirm("cargo_makepad".to_string())
                    .terminal()
                    .success();
                TerminalLogger::new("ℹ You can use `cargo makepad -h` to see the help information")
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
        if check_cargo_makepad() {
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
    fs::delete_dir(path.as_ref().join("makepad"))?;

    InstallLogs::Install("makepad".to_string())
        .terminal()
        .info();
    // use git clone makepad: git clone --branch gen_ui --depth 1 https://github.com/syf20020816/makepad.git makepad
    let mut child = Command::new("git")
        .args(&[
            "clone",
            "--branch",
            "gen_ui",
            "--depth",
            "1",
            "https://github.com/syf20020816/makepad.git",
            "makepad",
        ])
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
    let mut child = Command::new("git")
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
