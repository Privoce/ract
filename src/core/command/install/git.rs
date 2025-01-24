use gen_utils::error::Error;

#[cfg(target_os = "linux")]
pub fn install_git() -> Result<(), Error> {
    Err("Not support yet, in different Linux/Unix has multi ways to install git, please install git yourself".to_string().into())
}

#[cfg(target_os = "macos")]
pub fn install_git() -> Result<(), Error> {
    // first check brew exists
    use crate::core::log::InstallLogs;
    use crate::core::log::TerminalLogger;
    use std::process::Command;
    use which::which;

    let brew = which("brew").is_ok();
    if !brew {
        // install brew: /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        let _ = Command::new("/bin/bash")
            .args(&[
                "-c",
                "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)",
            ])
            .output()
            .map_or_else(
                |e| Err(Error::FromDynError(e.to_string())),
                |out| {
                    if out.status.success() {
                        TerminalLogger::new("✅ Install brew success").success();
                        Ok(())
                    } else {
                        Err(InstallLogs::InstallErr("brew".to_string())
                            .to_string()
                            .into())
                    }
                },
            )?;
    }

    // install git: brew install git
    Command::new("brew")
        .args(&["install", "git"])
        .output()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |out| {
                if out.status.success() {
                    InstallLogs::Git.terminal().success();
                    Ok(())
                } else {
                    Err(InstallLogs::InstallErr("git".to_string())
                        .to_string()
                        .into())
                }
            },
        )
}

#[cfg(target_os = "windows")]
pub fn install_git() -> Result<(), Error> {
    // https://github.com/git-for-windows/git/releases/download/v2.47.1.windows.1/Git-2.47.1-64-bit.exe
    use crate::core::log::TerminalLogger;
    use crate::core::util::exe_path;
    use std::process::Command;
    let current_dir = exe_path().join("downloads");
    let res = Command::new("Invoke-WebRequest")
        .args(&[
            "-Uri",
            "https://github.com/git-for-windows/git/releases/download/v2.47.1.windows.1/Git-2.47.1-64-bit.exe",
            "-OutFile",
            "git-installer.exe",
        ])
        .current_dir(current_dir.as_path())
        .output()
        .map_or_else(
            |e| Err(Error::FromDynError(e.to_string())),
            |_| {
                TerminalLogger::new("✅ Download git-installer.exe success").success();
                // run the git-installer.exe
                Command::new("git-installer.exe")
                    .current_dir(current_dir.as_path())
                    .output()
                    .map_or_else(
                        |e| Err(Error::FromDynError(e.to_string())),
                        |out| {
                            if out.status.success() {
                                InstallLogs::Git.terminal().success();
                                Ok(())
                            } else {
                                Err(InstallLogs::InstallErr("git".to_string())
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
