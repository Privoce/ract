use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::core::util::real_chain_env_toml;
use clap::ValueEnum;
use gen_utils::{
    common::{DepType, RustDependence, Source, ToToml},
    compiler::{CompilerImpl, UnderlayerConfImpl},
    error::{Error, ParseError},
};
use makepad_gen_plugin::compiler::{
    Compiler as MakepadCompiler, Config as MakepadConfig,
    CONF_FORMAT_SUGGESTION as MAKEPAD_CONF_FORMAT_SUGGESTION,
};
use toml_edit::{DocumentMut, Formatted, Item, Value};

use super::GenUIConf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Underlayer {
    #[default]
    Makepad,
}

impl Underlayer {
    pub fn options() -> Vec<&'static str> {
        vec!["Makepad"]
    }
    /// write a GenUI project's Conf: gen_ui.toml
    pub fn write_gen_ui_toml<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let toml = GenUIConf::try_from((path.as_ref().to_path_buf(), *self))?;
        toml.write(path.as_ref().join("gen_ui.toml"))
    }
    pub fn compiler(
        &self,
        source: &Source,
        conf: &Box<dyn UnderlayerConfImpl>,
    ) -> Result<Box<dyn CompilerImpl>, Error> {
        let compiler = match self {
            Underlayer::Makepad => MakepadCompiler::new(source.clone(), conf),
        }?;

        Ok(Box::new(compiler))
    }
}

impl FromStr for Underlayer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Makepad" | "makepad" => Ok(Underlayer::Makepad),
            _ => Err(format!("unknown target: {}", s).into()),
        }
    }
}

impl Display for Underlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&Underlayer> for Value {
    fn from(value: &Underlayer) -> Self {
        Value::String(Formatted::new(
            match value {
                Underlayer::Makepad => "makepad",
            }
            .to_string(),
        ))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CompileUnderlayer {
    pub target: Box<dyn UnderlayerConfImpl>,
    pub others: Option<Vec<Box<dyn UnderlayerConfImpl>>>,
}

impl TryFrom<(PathBuf, Underlayer)> for CompileUnderlayer {
    type Error = Error;

    fn try_from(value: (PathBuf, Underlayer)) -> Result<Self, Self::Error> {
        match value.1 {
            Underlayer::Makepad => Self::makepad(value.0),
        }
    }
}

impl TryFrom<(&DocumentMut, Underlayer)> for CompileUnderlayer {
    type Error = Error;

    fn try_from(value: (&DocumentMut, Underlayer)) -> Result<Self, Self::Error> {
        let (toml, target) = value;
        let target = match target {
            Underlayer::Makepad => toml.get("makepad").map_or_else(
                || Err(Error::from(MAKEPAD_CONF_FORMAT_SUGGESTION)),
                |table| MakepadConfig::try_from(table).and_then(|conf| Ok(Box::new(conf))),
            ),
        }?;

        Ok(Self {
            target,
            others: None,
        })
    }
}

impl CompileUnderlayer {
    /// default makepad toml
    fn makepad(root: PathBuf) -> Result<Self, Error> {
        let chain_env_toml = real_chain_env_toml()?;
        let makepad_dep_path = PathBuf::from_str(
            chain_env_toml["dependencies"]["makepad-widgets"]
                .as_str()
                .expect("makepad-widgets path not found"),
        )
        .map_err(|e| {
            Error::Parse(ParseError::new(
                e.to_string().as_str(),
                gen_utils::error::ParseType::Toml,
            ))
        })?;
        let mut makepad_dep = RustDependence::new("makepad-widgets");
        makepad_dep.set_ty(DepType::Local(makepad_dep_path.join("widgets")));
        // new makepad config and add dependence
        let mut conf = MakepadConfig::new(root.join("views").join("root.gen"));
        conf.push_dep(makepad_dep);
        let target = Box::new(conf);

        Ok(Self {
            target,
            others: None,
        })
    }
}

impl Display for CompileUnderlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.target.to_string())
    }
}

impl From<&CompileUnderlayer> for Item {
    fn from(value: &CompileUnderlayer) -> Self {
        (&*value.target).to_item()
    }
}
