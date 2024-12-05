use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use gen_utils::common::fs::path_to_str;
use toml_edit::{Formatted, InlineTable, Value};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl From<&Position> for Value {
    fn from(position: &Position) -> Self {
        let mut v = InlineTable::new();

        v.insert("x", Value::Integer(Formatted::new(position.x as i64)));
        v.insert("y", Value::Integer(Formatted::new(position.y as i64)));

        Value::InlineTable(v)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl From<&Size> for Value {
    fn from(size: &Size) -> Self {
        let mut v = InlineTable::new();

        v.insert("width", Value::Integer(Formatted::new(size.width as i64)));
        v.insert("height", Value::Integer(Formatted::new(size.height as i64)));

        Value::InlineTable(v)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

/// # BundleTypeRole
///
/// One of the following:
///
///     "editor" CFBundleTypeRole.Editor. Files can be read and edited.
///     "viewer" CFBundleTypeRole.Viewer. Files can be read.
///     "shell" CFBundleTypeRole.Shell
///     "qLGenerator" CFBundleTypeRole.QLGenerator
///     "none" CFBundleTypeRole.None
///
/// macOS-only*. Corresponds to CFBundleTypeRole
#[derive(Debug, Clone, Default, Copy)]
pub enum BundleTypeRole {
    #[default]
    Editor,
    Viewer,
    Shell,
    QLGenerator,
    None,
}

impl From<&BundleTypeRole> for Value {
    fn from(b_ty: &BundleTypeRole) -> Self {
        Value::String(Formatted::new(
            match b_ty {
                BundleTypeRole::Editor => "editor",
                BundleTypeRole::Viewer => "viewer",
                BundleTypeRole::Shell => "shell",
                BundleTypeRole::QLGenerator => "qLGenerator",
                BundleTypeRole::None => "none",
            }
            .to_string(),
        ))
    }
}

impl Display for BundleTypeRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

/// # PackageFormat
///
/// One of the following:
///
///     "all" All available package formats for the current platform. See [PackageFormat::platform_all]
///     "default" The default list of package formats for the current platform. See [PackageFormat::platform_default]
///     "app" The macOS application bundle (.app).
///     "dmg" The macOS DMG package (.dmg).
///     "wix" The Microsoft Software Installer (.msi) through WiX Toolset.
///     "nsis" The NSIS installer (.exe).
///     "deb" The Linux Debian package (.deb).
///     "appimage" The Linux AppImage package (.AppImage).
///     "pacman" The Linux Pacman package (.tar.gz and PKGBUILD)
///
/// Types of supported packages by cargo-packager.
#[derive(Debug, Clone, Default)]
pub enum PackageFormat {
    All,
    #[default]
    Default,
    App,
    Dmg,
    Wix,
    Nsis,
    Deb,
    AppImage,
    Pacman,
}

impl From<&PackageFormat> for Value {
    fn from(p_f: &PackageFormat) -> Self {
        Value::String(Formatted::new(
            match p_f {
                PackageFormat::All => "all",
                PackageFormat::Default => "default",
                PackageFormat::App => "app",
                PackageFormat::Dmg => "dmg",
                PackageFormat::Wix => "wix",
                PackageFormat::Nsis => "nsis",
                PackageFormat::Deb => "deb",
                PackageFormat::AppImage => "appimage",
                PackageFormat::Pacman => "pacman",
            }
            .to_string(),
        ))
    }
}

impl Display for PackageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

#[derive(Debug, Clone)]
pub enum Resource {
    String(String),
    Obj { src: PathBuf, target: String },
}

impl Resource {
    pub fn new_obj<P>(src: P, target: &str) -> Self
    where
        P: AsRef<Path>,
    {
        Self::Obj {
            src: src.as_ref().to_path_buf(),
            target: target.to_string(),
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&Resource> for Value {
    fn from(resource: &Resource) -> Self {
        match resource {
            Resource::String(s) => Value::String(Formatted::new(s.to_string())),
            Resource::Obj { src, target } => {
                let mut v = InlineTable::new();

                v.insert(
                    "src",
                    Value::String(Formatted::new(path_to_str(src))),
                );
                v.insert("target", Value::String(Formatted::new(target.to_string())));

                Value::InlineTable(v)
            }
        }
    }
}
