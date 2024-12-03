use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use gen_utils::{
    common::{fs, stream_terminal},
    error::Error,
};

use crate::core::log::TerminalLogger;

use super::{MacOsConfig, PackageConf};

pub struct PackageGenerator {
    path: PathBuf,
}

impl PackageGenerator {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// generate the packaging project for makepad
    pub fn makepad(&self, conf: PackageConf) -> Result<(), Error> {
        // [Packager.toml] -------------------------------------------------------------------
        let _ = fs::write(self.path.join("Packager.toml"), &conf.to_string())?;
        // git clone --branch ract  https://github.com/syf20020816/robius-packaging-commands.git command
        let mut child = Command::new("git")
            .args(&[
                "clone",
                "--branch",
                "ract",
                "https://github.com/syf20020816/robius-packaging-commands.git",
                "packaging",
            ])
            .current_dir(self.path.as_path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let _ = stream_terminal(
            &mut child,
            |line| TerminalLogger::new(&line).info(),
            |line| TerminalLogger::new(&line).warning(),
        )
        .map_or_else(
            |e| Err(e),
            |status| {
                if status.success() {
                    TerminalLogger::new("✅ packaging commands download success!").success();
                    Ok(())
                } else {
                    Err("❌ packaging resources download failed!".to_string().into())
                }
            },
        )?;

        // generate needed resources
        // [$name.desktop] -------------------------------------------------------------------
        let pkg_path = self.path.join("packaging");
        let _ = fs::write(
            pkg_path.join(format!("{}.desktop", &conf.name)),
            &DESKTOP.replace("${name}", &conf.name),
        );
        // [Entitlements.plist] --------------------------------------------------------------
        let _ = fs::write(
            pkg_path.join("Entitlements.plist"),
            &MacOsConfig::to_entitlements(),
        );
        // [macos_info.plist] ----------------------------------------------------------------
        let _ = fs::write(
            pkg_path.join("macos_info.plist"),
            &MacOsConfig::to_info_plist(&conf),
        );
        Ok(())
    }
}

const DESKTOP: &str = r#"[Desktop Entry]
Categories={{categories}}
{{#if comment}}
Comment={{comment}}
{{/if}}
Exec={{exec}} {{exec_arg}}
Icon={{icon}}
Name={{name}}
Terminal=false
Type=Application
StartupWMClass=${name}
{{#if mime_type}}
MimeType={{mime_type}}
{{/if}}
"#;
