use crate::entry::Language;
use crate::log::InstallLogs;
use gen_utils::error::Error;

#[cfg(target_os = "linux")]
pub fn install_rustc(lang: Language) -> Result<(), Error> {
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    use crate::log::{LogItem, LogExt};
    use gen_utils::common::stream_terminal;
    use std::process::Command;
    use std::process::Stdio;

    let mut child = Command::new("curl")
        .args(&[
            "--proto",
            "=https",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
            "|",
            "sh",
        ])
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
                InstallLogs::Installed("Rustc".to_string()).success(lang).print();
                Ok(())
            } else {
                Err(InstallLogs::InstallErr("rustc".to_string())
                    .to_string()
                    .into())
            }
        },
    )
}

#[cfg(target_os = "macos")]
pub fn install_rustc(lang: Language) -> Result<(), Error> {
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    use gen_utils::common::stream_terminal;
    use std::process::{Command, Stdio};
    use crate::log::{LogExt, LogItem};

    let mut child = Command::new("curl")
        .args(&[
            "--proto",
            "=https",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
            "|",
            "sh",
        ])
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
                InstallLogs::Installed("Rustc".to_string())
                    .success(lang)
                    .print();
                Ok(())
            } else {
                Err(InstallLogs::InstallErr("rustc".to_string())
                    .to_string()
                    .into())
            }
        },
    )
}

#[cfg(target_os = "windows")]
pub fn install_rustc(lang: Language) -> Result<(), Error> {
    // Powershell: Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile "rustup-init.exe"
    use crate::{log::{LogItem, LogExt}, common::exe_path};
    use std::process::Command;
    // create a downloads folder for the rustup-init.exe, after download, move to the exe_path
    let current_dir = exe_path()?.join("downloads");
    let res = Command::new("Invoke-WebRequest")
        .args(&[
            "-Uri",
            "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe",
            "-OutFile",
            "rustup-init.exe",
        ])
        .current_dir(current_dir.as_path())
        .output()
        .map_or_else(
            |e| Err(Error::FromDynError(e.to_string())),
            |_| {
                LogItem::success("✅ Download rustup-init.exe success".to_string()).print();
                // run the rustup-init.exe
                Command::new("rustup-init.exe")
                    .current_dir(current_dir.as_path())
                    .output()
                    .map_or_else(
                        |e| Err(Error::FromDynError(e.to_string())),
                        |out| {
                            if out.status.success() {
                                InstallLogs::Installed("Rustc".to_string()).success(lang).print();
                                Ok(())
                            } else {
                                Err(InstallLogs::InstallErr("rustc".to_string())
                                    .to_string()
                                    .into())
                            }
                        },
                    )
            },
        );

    // remove downloads folder (do not care if failed)
    let _ = std::fs::remove_dir_all(current_dir.as_path());
    res
}
