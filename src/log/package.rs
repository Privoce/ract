use std::{borrow::Cow, error::Error, fmt::Display};
use rust_i18n::t;

use super::LogExt;


#[derive(Debug, Clone)]
pub enum PackageLogs {
    Desc,
    Installed,
    UnInstalled,
    InstallErr(String),
    Init,
    Start,
    Confirm,
    PackageResourced,
    Error,
    Configing
}

impl Display for PackageLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.write_str(self.t(crate::entry::Language::En).as_ref())
    }
}

impl Error for PackageLogs {}

impl LogExt for PackageLogs {
    fn t(&self, lang: crate::entry::Language) -> Cow<str> {
        let lang_str = lang.as_str();
        match self {
            PackageLogs::Desc => t!("package.desc", locale = lang_str),
            PackageLogs::Installed => t!("package.installed", locale = lang_str),
            PackageLogs::UnInstalled => t!("package.uninstalled", locale = lang_str),
            PackageLogs::InstallErr(reason) => t!("package.install_err", locale = lang_str, reason = reason),
            PackageLogs::Init => t!("package.init", locale = lang_str),
            PackageLogs::Start => t!("package.start", locale = lang_str),
            PackageLogs::Confirm => t!("package.confirm", locale = lang_str),
            PackageLogs::PackageResourced => t!("package.resourced", locale = lang_str),
            PackageLogs::Error => t!("package.err", locale = lang_str),
            PackageLogs::Configing => t!("package.configing", locale = lang_str)
        }
    }
}