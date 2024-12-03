use crate::core::log::InstallLogs;
use gen_utils::error::Error;

#[cfg(target_os = "linux")]
pub fn install_rustc() -> Result<(), Error> {
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    use std::process::Command;
    Command::new("curl")
        .args(&[
            "--proto",
            "=https",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
            "|",
            "sh",
        ])
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Rustc.terminal().success();
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
pub fn install_rustc() -> Result<(), Error> {
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    use std::process::Command;
    Command::new("curl")
        .args(&[
            "--proto",
            "=https",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
            "|",
            "sh",
        ])
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Rustc.terminal().success();
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
pub fn install_rustc() -> Result<(), Error> {
    // Powershell: Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile "rustup-init.exe"
    use std::process::Command;
    use crate::core::env::exe_path;
    use crate::core::log::TerminalLogger;
    // create a downloads folder for the rustup-init.exe, after download, move to the exe_path
    let current_dir = exe_path().join("downloads");
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
                TerminalLogger::new("✅ Download rustup-init.exe success").success();
                // run the rustup-init.exe
                Command::new("rustup-init.exe")
                    .current_dir(current_dir.as_path())
                    .output()
                    .map_or_else(
                        |e| Err(Error::FromDynError(e.to_string())),
                        |out| {
                            if out.status.success() {
                                InstallLogs::Rustc.terminal().success();
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