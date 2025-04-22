use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    compiler::Version,
    error::{EnvError, Error},
};

use toml_edit::{value, DocumentMut, Formatted, InlineTable, Item, Table, Value};

use crate::{common::exe_path, log::TerminalLogger};

use super::env::Env;

/// # env.toml(chain_env_toml)
/// env.toml是Ract的环境配置文件，由这个文件中的内容Ract可以找到使用者开发时的环境依赖项。
pub struct ChainEnvToml {
    /// env.toml的路径
    pub path: PathBuf,
    /// Ract的版本(update命令需要辨别版本)
    pub version: Version,
    /// 标记是否是最新版本，用于提示用户更新
    pub is_latest: bool,
    /// 自动更新开关
    pub auto_update: bool,
    /// 检测
    pub check: Check,
    /// 依赖包
    pub dependencies: HashMap<String, PathBuf>,
    /// 语言
    pub language: Language,
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
    /// ## 检测是否需要更新
    /// 关闭了自动检测，则不需要检测
    /// 返回是否需要更新和当前版本，最新版本 (update?, (current_version, latest_version))
    pub fn check(
        &mut self,
    ) -> Result<(bool, Option<(String, String)>), Box<dyn std::error::Error>> {
        if self.check.check() {
            self.check_force()
        } else {
            Ok((false, None))
        }
    }

    /// ## 强制检测
    pub fn check_force(
        &mut self,
    ) -> Result<(bool, Option<(String, String)>), Box<dyn std::error::Error>> {
        // 获取最新版本
        let latest_version = search_latest_version()?;
        // 如果本地版本低于最新版本，则需要更新
        if latest_version > self.version {
            self.is_latest = false;
        } else {
            self.is_latest = true;
        }
        // 更新check的last_time, 用于下次检测, (目前默认为无论版本是否是旧版本都更新last_time, 用户可能不希望频繁检测)
        self.check.last_time = chrono::Utc::now().timestamp();
        // 写回env.toml
        self.write()?;
        Ok((
            !self.is_latest,
            Some((self.version.to_string(), latest_version.to_string())),
        ))
    }

    pub fn only_parse_lang() -> Result<Language, Error> {
        let path = Self::path()?;
        Self::read(path.as_path()).map_or_else(
            |_| Ok(Language::default()),
            |doc| {
                Ok(doc
                    .get("language")
                    .and_then(|v| v.as_str())
                    .map_or(Language::default(), |v| Language::from(v)))
            },
        )
    }
    pub fn lines_length(&self) -> usize {
        5 + self.dependencies.len()
    }
    pub fn to_lines(&self) -> Vec<(String, String, bool)> {
        let mut res = vec![
            (
                "auto_update".to_string(),
                self.auto_update.to_string(),
                true,
            ),
            (
                "language".to_string(),
                self.language.as_str().to_string(),
                true,
            ),
            ("[check]".to_string(), String::new(), false),
            (
                "frequency".to_string(),
                self.check.frequency.to_string(),
                true,
            ),
            ("[dependencies]".to_string(), String::new(), false),
        ];
        res.extend(
            self.dependencies
                .iter()
                .map(|(k, v)| (k.to_string(), fs::path_to_str(v.as_path()), true)),
        );

        res
    }
}

impl ToToml for ChainEnvToml {
    fn to_toml(&self) -> toml_edit::DocumentMut {
        let mut doc = DocumentMut::new();
        // [version] ------------------------------------------------------------------
        doc.insert("version", value(self.version.to_string()));
        // [is_latest] ----------------------------------------------------------------
        doc.insert("is_latest", value(self.is_latest));
        // [auto_update] --------------------------------------------------------------
        doc.insert("auto_update", value(self.auto_update));
        // [check] --------------------------------------------------------------------
        let mut check = InlineTable::new();
        check.insert("auto", Value::Boolean(Formatted::new(self.check.auto)));
        check.insert(
            "last_time",
            Value::Integer(Formatted::new(self.check.last_time)),
        );
        check.insert(
            "frequency",
            Value::Integer(Formatted::new(self.check.frequency)),
        );
        doc.insert("check", value(check));
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
        // [version] ------------------------------------------------------------------
        let version = doc.get("version").and_then(|v| v.as_str()).map_or_else(
            || {
                Err(EnvError::Get {
                    key: "version".to_string(),
                }
                .into())
            },
            |v| v.parse::<Version>(),
        )?;
        // [is_latest] ----------------------------------------------------------------
        let is_latest = doc
            .get("is_latest")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| EnvError::Get {
                key: "is_latest".to_string(),
            })?;
        // [auto_update] --------------------------------------------------------------
        let auto_update = doc
            .get("auto_update")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| EnvError::Get {
                key: "auto_update".to_string(),
            })?;

        // [check] --------------------------------------------------------------------
        let check = doc
            .get("check")
            .ok_or_else(|| EnvError::Get {
                key: "check".to_string(),
            })?
            .try_into()?;
        // [is_latest] ----------------------------------------------------------------
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

        // [language] ----------------------------------------------------------------
        let language = doc.get("language").map_or(Language::default(), |v| {
            v.as_str()
                .map_or(Language::default(), |v| Language::from(v))
        });

        Ok(Self {
            path,
            version,
            dependencies,
            is_latest,
            auto_update,
            check,
            language,
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
            is_latest: true,
            auto_update: true,
            check: Check::default(),
            language: Language::default(),
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
        Version::new(0, 1, 8)
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

pub struct Check {
    /// 是否自动检测更新
    pub auto: bool,
    /// 上次检测时间戳，用于判断是否需要检测更新，单位秒
    pub last_time: i64,
    /// 检测频率，单位秒，默认2天
    pub frequency: i64,
}

impl Default for Check {
    fn default() -> Self {
        Self {
            auto: true,
            last_time: chrono::Utc::now().timestamp() as i64,
            frequency: 2 * 24 * 60 * 60,
        }
    }
}

impl Check {
    /// ## 检测
    /// 1. 如果不是自动检测，则不需要检测, 返回false
    /// 2. 用当前时间戳 - 上次检测时间戳 > 检测频率 （则需要进行更新检测）
    pub fn check(&mut self) -> bool {
        if self.auto {
            let now = chrono::Utc::now().timestamp() as i64;
            now - self.last_time > self.frequency
        } else {
            false
        }
    }
}

impl TryFrom<&Item> for Check {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let value = value.as_inline_table().ok_or_else(|| {
            Error::Env(EnvError::Get {
                key: "check".to_string(),
            })
        })?;

        // [auto] ---------------------------------------------------------------------
        let auto = value.get("auto").and_then(|v| v.as_bool()).ok_or_else(|| {
            Error::Env(EnvError::Get {
                key: "[check.auto]".to_string(),
            })
        })?;
        // [last_time] ----------------------------------------------------------------
        let last_time = value
            .get("last_time")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| {
                Error::Env(EnvError::Get {
                    key: "[check.last_time]".to_string(),
                })
            })?;
        // [frequency] ----------------------------------------------------------------
        let frequency = value
            .get("frequency")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| {
                Error::Env(EnvError::Get {
                    key: "[check.frequency]".to_string(),
                })
            })?;

        Ok(Self {
            auto,
            last_time,
            frequency,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Language {
    Zh,
    #[default]
    En,
}

impl Language {
    pub fn from_conf() -> Language {
        ChainEnvToml::only_parse_lang().unwrap_or_default()
    }

    pub fn as_str(&self) -> &str {
        match self {
            Language::Zh => "zh_CN",
            Language::En => "en_US",
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "zh" | "zh_CN" | "zh-CN" => Language::Zh,
            "en" | "en_US" | "en-US" => Language::En,
            _ => Language::En,
        }
    }
}

/// 使用reqwest查询crate.io上的最新版本
fn search_latest_version() -> Result<Version, Box<dyn std::error::Error>> {
    TerminalLogger::new("🔍 Searching for the latest version...").info();
    let url = "https://crates.io/api/v1/crates/ract";
    // 增加header(user-agent)
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", "ract user")
        .send()?;

    let resp_json: serde_json::Value = response.json()?;
    let version = resp_json["crate"]["newest_version"]
        .as_str()
        .ok_or("version not found")?;
    Ok(version.parse::<Version>().unwrap())
}
