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
    entry::{Language, MakepadTools},
    log::{InstallLogs, LogExt, LogItem},
};

pub fn install<P>(
    path: P,
    makepad_is_ok: bool,
    makepad_tools: MakepadTools,
    lang: Language,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    update_makepad(makepad_is_ok, path.as_ref(), lang)?;

    match makepad_tools {
        crate::entry::MakepadTools::Makepad => {}
        crate::entry::MakepadTools::GenUi => {
            clone_gen_ui_components(path, lang)?;
        }
        crate::entry::MakepadTools::Android => {
            install_android_build(path, lang)?;
        }
        crate::entry::MakepadTools::Ios => {
            install_ios_build(path, lang)?;
        }
        crate::entry::MakepadTools::Wasm => {
            install_wasm_build(path, lang)?;
        }
        crate::entry::MakepadTools::Studio => {
            install_makepad_studio(path, lang)?;
        }
    }

    Ok(())
}

fn install_makepad_studio<P>(path: P, lang: Language) -> Result<(), Error>
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
                            .success(lang)
                            .print();
                        LogItem::info("".to_string()).print();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("makepad_studio".to_string())
                            .to_string()
                            .into())
                    }
                },
            );
    } else {
        InstallLogs::CargoMakepadErr.error(lang).print();
        exit(2);
    }
}

fn install_android_build<P>(path: P, lang: Language) -> Result<(), Error>
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
                            .success(lang)
                            .print();
                        LogItem::info("".to_string()).print();
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
        InstallLogs::CargoMakepadErr.error(lang).print();
        exit(2);
    }
}

fn install_ios_build<P>(path: P, lang: Language) -> Result<(), Error>
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
            InstallLogs::XCodeConfErr.error(lang).print();
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
                                .success(lang)
                                .print();
                            InstallLogs::MakepadIos.success(lang).multi().print();
                            Ok(())
                        } else {
                            Err(InstallLogs::InstallErr("ios_build".to_string())
                                .to_string()
                                .into())
                        }
                    },
                );
        } else {
            InstallLogs::CargoMakepadErr.error(lang).print();
            exit(2);
        }
    } else {
        InstallLogs::XCodeSelectErr.error(lang).print();
        exit(2);
    }
}

fn install_wasm_build<P>(path: P, lang: Language) -> Result<(), Error>
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
            |line| LogItem::info(line).print(),
            |line| LogItem::warning(line).print(),
        )
        .map_or_else(
            |e| Err(e),
            |status| {
                if status.success() {
                    InstallLogs::Confirm("wasm_build".to_string())
                        .success(lang)
                        .print();
                    InstallLogs::MakepadWasm.success(lang).print();
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

fn install_cargo_makepad<P>(path: P, lang: Language) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // cargo install --path ./tools/cargo_makepad
    InstallLogs::Install("cargo_makepad".to_string())
        .info(lang)
        .print();
    let mut child = Command::new("cargo")
        .args(&["install", "--path", "./makepad/tools/cargo_makepad"])
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
                InstallLogs::Confirm("cargo_makepad".to_string())
                    .success(lang)
                    .print();
                InstallLogs::MakepadHelp.success(lang).print();
                Ok(())
            } else {
                Err(InstallLogs::InstallErr("cargo_makepad".to_string())
                    .to_string()
                    .into())
            }
        },
    )
}

fn update_makepad<P>(install: bool, path: P, lang: Language) -> Result<(), Error>
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
                            clone_makepad(path.as_ref(), lang)?;
                            install_cargo_makepad(path, lang)
                        } else {
                            Ok(())
                        }
                    },
                );
        } else {
            InstallLogs::CargoMakepadErr.error(lang).print();
            exit(2);
        }
    } else {
        InstallLogs::MakepadWaitInstall.warning(lang).print();
        clone_makepad(path.as_ref(), lang)?;
        install_cargo_makepad(path, lang)
    }
}

fn clone_makepad<P>(path: P, lang: Language) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    fs::delete_dir(path.as_ref().join("makepad"))?;

    InstallLogs::Install("makepad".to_string())
        .info(lang)
        .print();
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
        |line| LogItem::info(line).print(),
        |line| LogItem::warning(line).print(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                InstallLogs::Confirm("makepad".to_string())
                    .success(lang)
                    .print();
                Ok(())
            } else {
                Err("❌ makepad install failed".to_string().into())
            }
        },
    )
}

fn clone_gen_ui_components<P>(path: P, lang: Language) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    InstallLogs::Install("gen_components".to_string())
        .info(lang)
        .print();
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
        |line| LogItem::info(line).print(),
        |line| LogItem::warning(line).print(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                InstallLogs::Confirm("gen_components".to_string())
                    .success(lang)
                    .print();

                Ok(())
            } else {
                Err("❌ gen_components install failed".to_string().into())
            }
        },
    )
}
