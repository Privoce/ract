use gen_utils::common::fs::path_to_str;
use gen_utils::error::{ConvertError, Error};
use std::str::FromStr;
use std::{fmt::Display, path::PathBuf};
use toml_edit::{Formatted, InlineTable, Value};

pub struct Binary {
    pub main: bool,
    pub path: PathBuf,
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&Binary> for Value {
    fn from(binary: &Binary) -> Self {
        let mut v = InlineTable::new();

        let path = path_to_str(&binary.path);
        #[cfg(target_os = "windows")]
        let path = path.replace("/", "\\");

        v.insert("main", Value::Boolean(Formatted::new(binary.main)));
        v.insert("path", Value::String(Formatted::new(path)));

        Value::InlineTable(v)
    }
}

impl TryFrom<&Value> for Binary {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_inline_table().map_or_else(
            || {
                Err(Error::Convert(ConvertError::FromTo {
                    from: "&toml_edit::Item".to_string(),
                    to: "Binary".to_string(),
                }))
            },
            |t| {
                let main = t.get("main").and_then(|v| v.as_bool()).map_or_else(
                    || {
                        Err(Error::Convert(ConvertError::FromTo {
                            from: "&toml_edit::Item".to_string(),
                            to: "bool".to_string(),
                        }))
                    },
                    |f| Ok(f),
                )?;

                let path = t.get("path").and_then(|v| v.as_str()).map_or_else(
                    || {
                        Err(Error::Convert(ConvertError::FromTo {
                            from: "&toml_edit::Item".to_string(),
                            to: "String".to_string(),
                        }))
                    },
                    |s| PathBuf::from_str(s).map_err(|e| Error::from(e.to_string())),
                )?;

                Ok(Self { main, path })
            },
        )
    }
}
