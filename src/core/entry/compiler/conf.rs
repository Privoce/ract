use std::{fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::fs,
    error::{ConvertError, Error},
};

use toml_edit::{DocumentMut, Item};

use crate::core::log::LogLevel;

use super::{target::CompileUnderlayer, CompileTarget};

/// Compiler Config for gen_ui.toml
/// ```toml
/// [compiler]
/// // see [CompileTarget]
/// [makepad]
/// // see [MakepadConfig]
/// ```
#[derive(Debug)]
pub struct CompilerConfigToml {
    pub compiler: CompilerConf,
    /// underlayer for makepad (current support)
    pub underlayer: CompileUnderlayer,
}

// get content and from toml path
impl TryFrom<&PathBuf> for CompilerConfigToml {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        fs::read(value)?.parse()
    }
}

impl FromStr for CompilerConfigToml {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // convert to DocumentMut
        let toml = s.parse::<DocumentMut>().map_err(|e| e.to_string())?;
        // [compiler] ------------------------------------------------------------------------------------------------
        let compiler = toml.get("compiler").map_or_else(
            || Ok(CompilerConf::default()),
            |table| CompilerConf::try_from(table),
        )?;

        let underlayer = CompileUnderlayer::try_from((&toml, compiler.target))?;

        Ok(Self {
            compiler,
            underlayer,
        })
    }
}

impl TryFrom<(PathBuf, CompileTarget)> for CompilerConfigToml {
    type Error = Error;

    fn try_from(value: (PathBuf, CompileTarget)) -> Result<Self, Self::Error> {
        let compiler = CompilerConf::default();

        Ok(Self {
            compiler,
            underlayer: value.try_into()?,
        })
    }
}

impl Display for CompilerConfigToml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}", self.compiler, self.underlayer))
    }
}


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
}

impl Default for CompilerConf {
    fn default() -> Self {
        Self {
            target: Default::default(),
            logo: true,
            log_level: Default::default(),
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

            return Ok(Self {
                target,
                logo,
                log_level,
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
        f.write_str("[compiler]\n")?;
        f.write_fmt(format_args!("target = \"{}\"\n", self.target))?;
        f.write_fmt(format_args!("logo = {}\n", self.logo))?;
        f.write_fmt(format_args!("log_level = \"{}\"\n", self.log_level))
    }
}


#[cfg(test)]
mod test_conf {
    use crate::core::entry::CompilerConfigToml;

    #[test]
    fn test_compiler_conf() {
        let toml = r#"
            [compiler]
            target = "makepad"
            logo = true
            log_level = "info"
            [makepad]
            root = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
        "#;
        let toml2 = r#"
            [compiler]
            target = "makepad"
            logo = false
            log_level = "error"

            [makepad]
            root = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
            entry = "hello"
            [makepad.wasm]
            fresh = true
        "#;
        let conf = toml.parse::<CompilerConfigToml>().unwrap();
        let conf2 = toml2.parse::<CompilerConfigToml>().unwrap();

        dbg!(conf);
        dbg!(conf2);
    }
}
