mod compiler;

pub use compiler::CompilerConf;
use std::{fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    error::Error,
};

use toml_edit::{DocumentMut, Item, Table};

use super::{target::CompileUnderlayer, Underlayer};

/// Compiler Config for gen_ui.toml
/// ```toml
/// [compiler]
/// // see [Underlayer]
/// [makepad]
/// // see [MakepadConfig]
/// ```
#[derive(Debug)]
pub struct Conf {
    pub compiler: CompilerConf,
    /// underlayer for makepad (current support)
    pub underlayer: CompileUnderlayer,
}

impl ToToml for Conf {
    fn to_toml(&self) -> DocumentMut {
        let mut table = Table::new();

        table.insert("compiler", (&self.compiler).into());
        // underlayer is a table which only has one node
        if let Item::Table(underlayer) = Item::from(&self.underlayer) {
            let (k, v) = underlayer.into_iter().next().unwrap();
            table.insert(&k, v);
        }

        table.set_implicit(false);
        DocumentMut::from(table)
    }
}

// get content and from toml path
impl TryFrom<&PathBuf> for Conf {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        fs::read(value)?.parse()
    }
}

impl FromStr for Conf {
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

impl TryFrom<(PathBuf, Underlayer)> for Conf {
    type Error = Error;

    fn try_from(value: (PathBuf, Underlayer)) -> Result<Self, Self::Error> {
        let compiler = CompilerConf::default();

        Ok(Self {
            compiler,
            underlayer: value.try_into()?,
        })
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl TryFrom<&DocumentMut> for Conf {
    type Error = Error;

    fn try_from(value: &DocumentMut) -> Result<Self, Self::Error> {
        let compiler_section = value.get("compiler").map_or_else(
            || Ok(CompilerConf::default()),
            |table| CompilerConf::try_from(table),
        )?;

        let underlayer_section = CompileUnderlayer::try_from((value, compiler_section.target))?;

        Ok(Self {
            compiler: compiler_section,
            underlayer: underlayer_section,
        })
    }
}

#[cfg(test)]
mod test_conf {
    use super::*;

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
            excludes = ["Cargo.toml"]

            [makepad]
            root = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
            entry = "hello"
            [makepad.dependencies]
            makepad_widgets = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
            [makepad.wasm]
            fresh = true
        "#;
        let conf = toml.parse::<Conf>().unwrap();
        let conf2 = toml2.parse::<Conf>().unwrap();

        println!("{}", conf);
        println!("{}", conf2);
    }
}
