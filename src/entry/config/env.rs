use std::{fmt::Display, path::{Path, PathBuf}, str::FromStr};

use gen_utils::{common::fs, error::Error};

use crate::common::exe_path;

use super::ChainEnvToml;

/// Ract的环境配置指向文件(.env)
/// 这个文件指向了Ract的环境配置文件(env.toml)当然也可以自定义名字
pub struct Env(pub PathBuf);

impl Env {
    /// 检查.env文件是否存在或内容是否正确
    pub fn check() -> bool {
        Self::read().is_ok()
    }

    /// 读取.env文件
    pub fn read() -> Result<Self, Error> {
        let path = Self::path()?;
        let env_content = fs::read(path.as_path())?;
        let env: Env = env_content.as_str().try_into()?;
        Ok(env)
    }

    /// 写回.env文件
    pub fn write(&self) -> Result<(), Error> {
        let content = self.to_string();
        let path = Self::path()?;
        fs::write(path.as_path(), &content)
    }

    /// 获取.env文件路径
    pub fn path() -> Result<PathBuf, Error> {
        let exe_path = exe_path()?;
        Ok(exe_path.join(".env"))
    }

    pub fn set<P>(&mut self, path: P) where P: AsRef<Path>{
        self.0 = path.as_ref().to_path_buf();
    }
}

impl TryFrom<&str> for Env {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        PathBuf::from_str(value)
            .map(|path| Env(path))
            .map_err(|e| e.to_string().into())
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&fs::path_to_str(self.0.as_path()))
    }
}

impl Default for Env {
    fn default() -> Self {
        Env(ChainEnvToml::default_chain().path())
    }
}
