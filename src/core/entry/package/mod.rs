/// package binary
mod binary;
/// package category
mod category;
#[allow(dead_code)]
mod common;
mod conf;
mod deb;
mod dmg;
mod file;
mod generator;
mod macos;
mod nsis;
mod pacman;
mod windows;
mod wix;

pub use binary::Binary;
pub use category::AppCategory;
pub use common::*;
pub use conf::Conf as PackageConf;
pub use deb::DebianConfig;
pub use dmg::DmgConfig;
pub use file::*;
pub use macos::MacOsConfig;
pub use nsis::NsisConfig;
pub use pacman::PacmanConfig;
pub use windows::WindowsConfig;
pub use wix::WixConfig;
