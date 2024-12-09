use std::{fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{DepType, RustDependence},
    error::Error,
};
use toml_edit::{value, Item, Table};

use crate::core::util::real_chain_env_toml;

#[derive(Debug, Clone, Copy, Default)]
pub enum FrameworkType {
    #[default]
    GenUI,
    Makepad,
}

impl Display for FrameworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkType::GenUI => f.write_str("gen_ui"),
            FrameworkType::Makepad => f.write_str("makepad"),
        }
    }
}

impl FrameworkType {
    pub fn options() -> Vec<&'static str> {
        vec!["gen_ui", "makepad"]
    }
    pub fn is_gen_ui(&self) -> bool {
        matches!(self, FrameworkType::GenUI)
    }
    /// dependencies section in Cargo.toml
    /// - GenUI: None
    /// - Makepad: Some(makepad-widgets)
    pub fn dependencies(&self) -> Result<Item, Error> {
        match self {
            FrameworkType::GenUI => Err(Error::from("GenUI has no dependencies")),
            FrameworkType::Makepad => {
                let mut toml = Table::new();
                // read dependencies from ract chain
                let env_toml = real_chain_env_toml()?;
                let makepad_widgets = env_toml
                    .get("dependencies")
                    .and_then(|deps| deps.get("makepad-widgets").and_then(|value| value.as_str()))
                    .map_or_else(
                        || {
                            Err(Error::from(
                                "can not find [dependencies.makepad-widgets] in env.toml",
                            ))
                        },
                        |s| Ok(s.to_string()),
                    )?;
                let makepad_widgets_path = PathBuf::from(makepad_widgets).join("widgets");
                let mut rust_dep = RustDependence::new("makepad-widgets");
                let _ = rust_dep.set_ty(DepType::local(makepad_widgets_path));
                let (key, item) = rust_dep.to_table_kv();
                // set dependencies to toml
                toml.insert(&key, item);

                Ok(Item::Table(toml))
            }
        }
    }
}

impl From<FrameworkType> for Item {
    fn from(f: FrameworkType) -> Self {
        value(f.to_string())
    }
}

impl FromStr for FrameworkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gen_ui" => Ok(FrameworkType::GenUI),
            "makepad" => Ok(FrameworkType::Makepad),
            _ => Err(Error::from("FrameworkType not found")),
        }
    }
}
