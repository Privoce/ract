mod compiler;

pub use compiler::CompilerConf;
use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    error::{ConvertError, Error},
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
    /// genui plugins, each plugin has a token.toml file
    pub plugins: Option<HashMap<String, PathBuf>>,
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

        let plugins = if let Some(plugins) = self.plugins.as_ref() {
            let mut table = Table::new();
            for (k, v) in plugins.iter() {
                table.insert(k, toml_edit::value(&fs::path_to_str(v)));
            }

            Item::Table(table)
        } else {
            Item::None
        };

        table.insert("plugins", plugins);

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
        s.parse::<DocumentMut>()
            .map_err(|e| e.to_string())?
            .try_into()
    }
}

impl TryFrom<(PathBuf, Underlayer)> for Conf {
    type Error = Error;

    fn try_from(value: (PathBuf, Underlayer)) -> Result<Self, Self::Error> {
        let compiler = CompilerConf::default();

        Ok(Self {
            compiler,
            underlayer: value.try_into()?,
            plugins: None,
        })
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl TryFrom<DocumentMut> for Conf {
    type Error = Error;

    fn try_from(toml: DocumentMut) -> Result<Self, Self::Error> {
        // [compiler] ------------------------------------------------------------------------------------------------
        let compiler = toml.get("compiler").map_or_else(
            || Ok(CompilerConf::default()),
            |table| CompilerConf::try_from(table),
        )?;
        // [plugins] -------------------------------------------------------------------------------------------------
        let plugins = if let Some(item) = toml.get("plugins") {
            let plugins = item.as_table().map_or_else(
                || {
                    Err(Error::from(ConvertError::FromTo {
                        from: "toml::Item".to_string(),
                        to: "toml::Table, gen_ui.toml [plugins]".to_string(),
                    }))
                },
                |table| {
                    let mut map = HashMap::new();
                    for (k, v) in table.iter() {
                        let path = v.as_str().map_or_else(
                            || {
                                Err(Error::from(ConvertError::FromTo {
                                    from: "toml::Item".to_string(),
                                    to: "toml::String, gen_ui.toml [plugins]".to_string(),
                                }))
                            },
                            |s| Ok(PathBuf::from(s)),
                        )?;

                        map.insert(k.to_string(), path);
                    }

                    Ok(map)
                },
            )?;

            if plugins.is_empty() {
                None
            } else {
                Some(plugins)
            }
        } else {
            None
        };
        // [underlayer] -----------------------------------------------------------------------------------------------
        let underlayer = CompileUnderlayer::try_from((toml, compiler.target))?;

        Ok(Self {
            compiler,
            underlayer,
            plugins,
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
