use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use gen_utils::{
    common::{fs, git_download_from_github, stream_terminal, ToToml},
    error::Error,
};
use toml_edit::DocumentMut;

use crate::core::{entry::FrameworkType, log::TerminalLogger};

use super::{MacOsConfig, PackageConf};

/// A generator for packaging project
/// This generator will generate the packaging project and resources for makepad or others
pub struct Generator {
    path: PathBuf,
}

impl Generator {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    // pub fn generate(&self, target: FrameworkType, conf: PackageConf) -> Result<(), Error> {
    //     match target {
    //         FrameworkType::GenUI => unimplemented!("GenUI packaging is not supported yet"),
    //         FrameworkType::Makepad => self.makepad(conf),
    //     }
    // }

    /// patch package configuration to Cargo.toml
    fn patch_to_cargo_toml(&self, conf: &PackageConf) -> Result<(), Error> {
        let path = self.path.join("Cargo.toml");
        let mut cargo_toml = fs::read(path.as_path())?
            .parse::<DocumentMut>()
            .map_err(|e| e.to_string())?;
        let (key, value) = conf.as_table_section();
        cargo_toml.insert(&key, value);
        // write back to Cargo.toml
        fs::write(path.as_path(), &cargo_toml.to_string())
    }

    /// handle resources for packaging
    /// 1. generate needed resources
    fn handle_resources(&self, conf: PackageConf) -> Result<(), Error> {
        // [git download from github] --------------------------------------------------------
        let tmp_path = self.path.join(".tmp");
        let from_path = tmp_path.join("resources").join("package");
        let to_path = self.path.join("package");
        let _ = git_download_from_github(
            "https://github.com/Privoce/ract.git", 
            "dev_v0.1.3", 
            "resources/package/*",
            self.path.as_path(), 
            |line| TerminalLogger::new(&line).info(), 
            |line| TerminalLogger::new(&line).warning()
        )?;
        // - [move resources] ----------------------------------------------------------------
        fs::move_to(from_path, to_path)?;
        fs::delete_dir(tmp_path)?;
        // [makepad a package dir for packaging resources] -----------------------------------
        let pkg_path = self.path.join("package");
        // [generate needed resources] -------------------------------------------------------
        // - [$name.desktop] -----------------------------------------------------------------
        // - [Entitlements.plist] ------------------------------------------------------------
        // - [macos_info.plist] --------------------------------------------------------------
        for (path, content) in [
            (
                pkg_path.join(format!("{}.desktop", &conf.name)),
                DESKTOP.replace("${name}", &conf.name),
            ),
            (
                pkg_path.join("Entitlements.plist"),
                MacOsConfig::to_entitlements(),
            ),
            (
                pkg_path.join("macos_info.plist"),
                MacOsConfig::to_info_plist(&conf),
            ),
        ] {
            let _ = fs::write(path, &content);
        }



        Ok(())
    }

    /// generate the packaging needed resources
    pub fn generate(&self, conf: PackageConf) -> Result<(), Error> {
        // [patch to Cargo] -------------------------------------------------------------------
        self.patch_to_cargo_toml(&conf)?;
        // [handle resources] -----------------------------------------------------------------
        self.handle_resources(conf)
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
