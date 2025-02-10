use super::{ChainEnvToml, Resource};
use gen_utils::{
    common::{DepType, RustDependence},
    error::Error,
};
use std::{fmt::Display, str::FromStr};
use toml_edit::{value, Item, Table};

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
    pub fn resources_in_ract(&self) -> Vec<Resource> {
        match self {
            FrameworkType::GenUI => {
                vec![
                    Resource::new_obj("gen_components", "../dist/resources/gen_components"),
                    Resource::new_obj("makepad-widgets", "../dist/resources/makepad_widgets"),
                ]
            }
            FrameworkType::Makepad => vec![Resource::new_obj(
                "makepad-widgets",
                "./dist/resources/makepad_widgets",
            )],
        }
    }
    /// back all copy items
    pub fn copys() -> Vec<&'static str> {
        vec!["gen_components", "makepad-widgets"]
    }
    /// copy items for different framework (resources for packaging)
    pub fn copy_items(&self) -> Vec<&'static str> {
        match self {
            FrameworkType::GenUI => Self::copys(),
            FrameworkType::Makepad => vec!["makepad-widgets"],
        }
    }
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
                let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
                let makepad_widgets_path = chain_env_toml
                .makepad_widgets_path()
                .map_or_else(||Err(
                    Error::from(
                        "can not find [dependencies.makepad-widgets] in env.toml, maybe config broken, use `ract init` to fix it",
                    )
                ), |path| Ok(path.join("widgets")))?;
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
