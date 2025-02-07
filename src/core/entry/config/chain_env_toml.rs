use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    compiler::Version,
    error::{EnvError, Error},
};
use toml_edit::{value, DocumentMut, Item, Table};

use crate::core::util::exe_path;

use super::env::Env;

/// # env.toml(chain_env_toml)
/// env.toml是Ract的环境配置文件，由这个文件中的内容Ract可以找到使用者开发时的环境依赖项。
pub struct ChainEnvToml {
    /// env.toml的路径
    pub path: PathBuf,
    /// Ract的版本(update命令需要辨别版本)
    pub version: Version,
    /// 依赖包
    pub dependencies: HashMap<String, PathBuf>,
}

impl ChainEnvToml {
    pub fn path() -> Result<PathBuf, Error> {
        Ok(Env::read()?.0)
    }
    pub fn makepad_widgets_path(&self) -> Option<&PathBuf> {
        self.dependencies.get("makepad-widgets")
    }
    pub fn gen_components_path(&self) -> Option<&PathBuf> {
        self.dependencies.get("gen_components")
    }
    pub fn default_chain() -> DefaultChain {
        DefaultChain
    }
    pub fn write(&self) -> Result<(), Error> {
        fs::write(self.path.as_path(), self.to_string().as_str())
    }
    pub fn options() -> Vec<&'static str> {
        vec!["makepad-widgets", "gen_components"]
    }
    pub fn chain_path(&self) -> PathBuf {
        let mut path = self.path.to_path_buf();
        path.pop();
        path
    }
}

impl ToToml for ChainEnvToml {
    fn to_toml(&self) -> toml_edit::DocumentMut {
        let mut doc = DocumentMut::new();
        // [version] ------------------------------------------------------------------
        doc.insert("version", value(self.version.to_string()));
        // [dependencies] -------------------------------------------------------------
        let mut dependencies = Table::new();
        for (k, v) in self.dependencies.iter() {
            dependencies.insert(k, value(fs::path_to_str(v.as_path())));
        }
        doc.insert("dependencies", Item::Table(dependencies));
        doc
    }
}

impl TryFrom<PathBuf> for ChainEnvToml {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let doc = Self::read(path.as_path())?;
        let version = doc.get("version").and_then(|v| v.as_str()).map_or_else(
            || {
                Err(EnvError::Get {
                    key: "version".to_string(),
                }
                .into())
            },
            |v| v.parse::<Version>(),
        )?;
        let dependencies = doc
            .get("dependencies")
            .and_then(|v| v.as_table())
            .map_or_else(
                || {
                    Err(Error::Env(EnvError::Get {
                        key: "dependencies".to_string(),
                    }))
                },
                |v| {
                    let mut dependencies = HashMap::new();
                    for (k, v) in v.iter() {
                        let dep = v.as_str().map_or_else(
                            || Err(Error::Env(EnvError::Get { key: k.to_string() })),
                            |v| Ok(v.to_string()),
                        )?;

                        dependencies.insert(
                            k.to_string(),
                            PathBuf::from_str(&dep).map_err(|e| e.to_string())?,
                        );
                    }
                    Ok(dependencies)
                },
            )?;

        Ok(Self {
            path,
            version,
            dependencies,
        })
    }
}

impl Display for ChainEnvToml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl Default for ChainEnvToml {
    fn default() -> Self {
        let chain = DefaultChain;

        Self {
            path: chain.path(),
            version: chain.version(),
            dependencies: chain.dependencies(),
        }
    }
}

pub struct DefaultChain;

#[allow(unused)]
impl DefaultChain {
    pub fn makepad_widgets(&self) -> PathBuf {
        exe_path()
            .expect("exe path not found")
            .join("chain")
            .join("makepad")
    }
    pub fn gen_components(&self) -> PathBuf {
        exe_path()
            .expect("exe path not found")
            .join("chain")
            .join("gen_components")
    }
    pub fn version(&self) -> Version {
        Version::new(0, 1, 3)
    }
    pub fn dependencies(&self) -> HashMap<String, PathBuf> {
        let chain = DefaultChain;
        let mut dependencies = HashMap::new();
        dependencies.insert("makepad-widgets".to_string(), chain.makepad_widgets());
        dependencies.insert("gen_components".to_string(), chain.gen_components());
        dependencies
    }
    pub fn path(&self) -> PathBuf {
        exe_path()
            .expect("exe path not found")
            .join("chain")
            .join("env.toml")
    }
}
