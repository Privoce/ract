use std::fmt::Display;

use gen_utils::error::{ConvertError, Error};
use toml_edit::{value, Item, Table};

use crate::core::{
    entry::{compiler::excludes::Excludes, CompileTarget},
    log::LogLevel,
};

/// Compiler Config
/// ```toml
/// [compiler]
/// target = "makepad"
/// logo = true
/// log_level = "info"
/// ```
#[derive(Debug)]
pub struct CompilerConf {
    pub target: CompileTarget,
    pub logo: bool,
    pub log_level: LogLevel,
    pub excludes: Excludes,
}

impl Default for CompilerConf {
    fn default() -> Self {
        Self {
            target: Default::default(),
            logo: true,
            log_level: Default::default(),
            excludes: Default::default(),
        }
    }
}

impl TryFrom<&Item> for CompilerConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        if let Some(table) = value.as_table() {
            let target = table
                .get("target")
                .and_then(|item| item.as_str())
                .map_or_else(|| Ok(Default::default()), |s| s.parse())?;
            let logo = table
                .get("logo")
                .and_then(|item| item.as_bool())
                .unwrap_or_default();
            let log_level = table
                .get("log_level")
                .and_then(|item| item.as_str())
                .map_or_else(|| Ok(Default::default()), |s| s.parse())?;

            let excludes = table
                .get("excludes")
                .and_then(|item| item.as_array())
                .map_or_else(|| Ok(Default::default()), |array| array.try_into())?;

            return Ok(Self {
                target,
                logo,
                log_level,
                excludes,
            });
        }

        Err(ConvertError::FromTo {
            from: "toml::Item".to_string(),
            to: "toml::Table, toml format not correct".to_string(),
        }
        .into())
    }
}

impl Display for CompilerConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl From<&CompilerConf> for Item {
    fn from(conf: &CompilerConf) -> Self {
        let mut table = Table::new();
        table.insert("target", Item::Value((&conf.target).into()));
        table.insert("logo", value(conf.logo));
        table.insert("log_level", Item::Value((&conf.log_level).into()));
        table.insert("excludes", Item::Value((&conf.excludes).into()));
        Item::Table(table)
    }
}
