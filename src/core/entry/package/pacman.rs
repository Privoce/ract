use std::fmt::Display;

use toml_edit::{value, Array, Table};

/// # PacmanConfig
///
/// The Linux pacman configuration.
#[derive(Debug, Clone)]
pub struct PacmanConfig {
    /// Packages that conflict or cause problems with the app.
    /// All these packages and packages providing this item will need to be removed
    pub conflicts: Option<Vec<String>>,
    /// List of softwares that must be installed for the app to build and run.
    pub depends: Option<Vec<String>>,
    /// List of custom files to add to the pacman package. Maps a dir/file to a dir/file inside the pacman package.
    pub files: Option<String>,
    /// Additional packages that are provided by this app.
    pub provides: Option<Vec<String>>,
    /// Only use if this app replaces some obsolete packages. For example, if you rename any package.
    pub replaces: Option<Vec<String>>,
    /// Source of the package to be stored at PKGBUILD. PKGBUILD is a bash script, so version can be referred as ${pkgver}
    pub source: Option<Vec<String>>,
}

impl Display for PacmanConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml_table().to_string().as_str())
    }
}

impl PacmanConfig {
    pub fn to_toml_table(&self) -> Table {
        let mut table = Table::new();

        if let Some(conflicts) = self.conflicts.as_ref() {
            let mut arr = Array::new();
            for c in conflicts {
                arr.push(c);
            }
            table.insert("conflicts", value(arr));
        }

        if let Some(depends) = self.depends.as_ref() {
            let mut arr = Array::new();
            for d in depends {
                arr.push(d);
            }
            table.insert("depends", value(arr));
        }

        if let Some(files) = self.files.as_ref() {
            table.insert("files", value(files));
        }

        if let Some(provides) = self.provides.as_ref() {
            let mut arr = Array::new();
            for p in provides {
                arr.push(p);
            }
            table.insert("provides", value(arr));
        }

        if let Some(replaces) = self.replaces.as_ref() {
            let mut arr = Array::new();
            for r in replaces {
                arr.push(r);
            }
            table.insert("replaces", value(arr));
        }

        if let Some(source) = self.source.as_ref() {
            let mut arr = Array::new();
            for s in source {
                arr.push(s);
            }
            table.insert("source", value(arr));
        }
        table.set_implicit(false);
        table
    }
}
