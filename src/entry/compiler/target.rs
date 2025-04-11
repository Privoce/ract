use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};
use clap::ValueEnum;
use gen_utils::{
    common::{DepType, RustDependence, Source, ToToml},
    compiler::{CompilerImpl, UnderlayerConfImpl},
    error::Error,
};
use makepad_gen_plugin::compiler::{
    Compiler as MakepadCompiler, Config as MakepadConfig,
    CONF_FORMAT_SUGGESTION as MAKEPAD_CONF_FORMAT_SUGGESTION,
};
use toml_edit::{DocumentMut, Formatted, InlineTable, Item, Value};
use crate::entry::ChainEnvToml;
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

impl TryFrom<(DocumentMut, Underlayer)> for CompileUnderlayer {
    type Error = Error;

    fn try_from(value: (DocumentMut, Underlayer)) -> Result<Self, Self::Error> {
        let (mut toml, target) = value;
        let target = match target {
            Underlayer::Makepad => toml.get_mut("makepad").map_or_else(
                || Err(Error::from(MAKEPAD_CONF_FORMAT_SUGGESTION)),
                |table| {
                    // before to makepad config, add gen_components dependence into [makepad.dependencies]
                    let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
                    let gen_dep_path = chain_env_toml.gen_components_path().map_or_else(
                        || Err(Error::from("can not find [dependencies.gen_components] in env.toml, maybe config broken, use `ract init` to fix it")),
                        |f| Ok(f),
                    )?;

                    table
                        .as_table_mut()
                        .unwrap()
                        .get_mut("dependencies")
                        .as_mut()
                        .map(|deps| {
                            let mut dep_table = InlineTable::new();
                            dep_table.insert(
                                "path",
                                Value::String(Formatted::new(
                                    gen_dep_path.to_str().unwrap().to_string(),
                                )),
                            );

                            deps.as_table_mut().unwrap().insert(
                                "gen_components",
                                Item::Value(Value::InlineTable(dep_table)),
                            );
                        });

                    MakepadConfig::try_from(table).and_then(|conf| Ok(Box::new(conf)))
                },
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
        let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
        let makepad_dep_path = chain_env_toml.makepad_widgets_path().map_or_else(
            || Err(Error::from("can not find [dependencies.makepad-widgets] in env.toml, maybe config broken, use `ract init` to fix it")),
            |f| Ok(f),
        )?;

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
