use std::{fmt::Display, path::PathBuf};

use gen_utils::common::fs::path_to_str;
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
