use std::{fmt::Display, path::PathBuf};

use toml_edit::{value, Formatted, InlineTable, Value};

pub struct Binary {
    pub main: bool,
    pub path: PathBuf,
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(value(self).to_string().as_str())
    }
}

impl Into<Value> for &Binary {
    fn into(self) -> Value {
        let mut v = InlineTable::new();

        v.insert("main", Value::Boolean(Formatted::new(self.main)));
        v.insert(
            "path",
            Value::String(Formatted::new(self.path.display().to_string())),
        );

        v.into()
    }
}
