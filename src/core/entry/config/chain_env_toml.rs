use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    compiler::Version,
    error::{EnvError, Error},
};
use toml_edit::{value, DocumentMut, Item, Table};

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
