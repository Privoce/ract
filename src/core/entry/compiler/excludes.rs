use std::path::{Path, PathBuf};

use gen_utils::{common::fs, error::Error};
use toml_edit::{Array, Formatted, Value};

/// # Gen Excludes
/// These files and directories are excludesd by the compiler(watcher)
/// Which need to write in `gen_ui.toml` file
/// ## Example
/// ```toml
/// [compiler]
/// excludes: ["Cargo.toml", "Cargo.lock", "src/main.rs", "target", ".gen_ui_cache"]
/// ```
/// ## Default Excludes
/// ["Cargo.toml", "Cargo.lock", "src/main.rs", "target", ".gen_ui_cache"]
#[derive(Debug, Clone)]
pub struct Excludes(pub Vec<PathBuf>);

impl Excludes {
    pub fn contains<P>(&self, prefix: P, path: P) -> bool
    where
        P: AsRef<Path>,
    {
        self.0.iter().any(|p| {
            let p = prefix.as_ref().join(p);
            // || path.as_ref().starts_with(&p)
            p == path.as_ref()
        })
    }
}

impl From<Excludes> for Vec<PathBuf> {
    fn from(value: Excludes) -> Self {
        value.0
    }
}

impl Default for Excludes {
    fn default() -> Self {
        Self(vec![
            PathBuf::from("Cargo.toml"),
            PathBuf::from("Cargo.lock"),
            PathBuf::from("src").join("main.rs"),
            PathBuf::from("target"),
            PathBuf::from(".gen_ui_cache"),
        ])
    }
}

impl TryFrom<&Array> for Excludes {
    type Error = Error;

    fn try_from(value: &Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|item| {
                item.as_str()
                    .map(|s| PathBuf::from(s))
                    .ok_or_else(|| Error::from("Excludes must be a string"))
            })
            .collect::<Result<Vec<PathBuf>, Error>>()
            .map(|v| Excludes(v))
    }
}

impl Into<Value> for &Excludes {
    fn into(self) -> Value {
        Value::Array(
            self.0
                .iter()
                .map(|p| Value::String(Formatted::new(fs::path_to_str(p))))
                .collect(),
        )
    }
}
