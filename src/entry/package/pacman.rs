use std::fmt::Display;

use gen_utils::error::{ParseError, ParseType};
use toml_edit::{value, Array, Item, Table};

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
        f.write_str(Item::from(self).to_string().as_str())
    }
}
impl From<&PacmanConfig> for Item {
    fn from(v: &PacmanConfig) -> Self {
        let mut table = Table::new();

        if let Some(conflicts) = v.conflicts.as_ref() {
            let mut arr = Array::new();
            for c in conflicts {
                arr.push(c);
            }
            table.insert("conflicts", value(arr));
        }

        if let Some(depends) = v.depends.as_ref() {
            let mut arr = Array::new();
            for d in depends {
                arr.push(d);
            }
            table.insert("depends", value(arr));
        }

        if let Some(files) = v.files.as_ref() {
            table.insert("files", value(files));
        }

        if let Some(provides) = v.provides.as_ref() {
            let mut arr = Array::new();
            for p in provides {
                arr.push(p);
            }
            table.insert("provides", value(arr));
        }

        if let Some(replaces) = v.replaces.as_ref() {
            let mut arr = Array::new();
            for r in replaces {
                arr.push(r);
            }
            table.insert("replaces", value(arr));
        }

        if let Some(source) = v.source.as_ref() {
            let mut arr = Array::new();
            for s in source {
                arr.push(s);
            }
            table.insert("source", value(arr));
        }
        table.set_implicit(false);
        Item::Table(table)
    }
}

impl TryFrom<&Item> for PacmanConfig {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let mut conflicts = None;
        let mut depends = None;
        let mut files = None;
        let mut provides = None;
        let mut replaces = None;
        let mut source = None;

        if let Item::Table(table) = value {
            for (k, v) in table.iter() {
                match k {
                    "conflicts" => {
                        let mut confs = Vec::new();
                        if let Item::Value(v) = v {
                            if let Some(arr) = v.as_array() {
                                for a in arr.iter() {
                                    a.as_str().map(|s| confs.push(s.to_string()));
                                }
                            }
                        }
                        conflicts = Some(confs);
                    }
                    "depends" => {
                        let mut deps = Vec::new();
                        if let Item::Value(v) = v {
                            if let Some(arr) = v.as_array() {
                                for a in arr.iter() {
                                    a.as_str().map(|s| deps.push(s.to_string()));
                                }
                            }
                        }
                        depends = Some(deps);
                    }
                    "files" => {
                        files = v.as_str().map(|s| s.to_string());
                    }
                    "provides" => {
                        let mut provs = Vec::new();
                        if let Item::Value(v) = v {
                            if let Some(arr) = v.as_array() {
                                for a in arr.iter() {
                                    a.as_str().map(|s| provs.push(s.to_string()));
                                }
                            }
                        }
                        provides = Some(provs);
                    }
                    "replaces" => {
                        let mut reps = Vec::new();
                        if let Item::Value(v) = v {
                            if let Some(arr) = v.as_array() {
                                for a in arr.iter() {
                                    a.as_str().map(|s| reps.push(s.to_string()));
                                }
                            }
                        }
                        replaces = Some(reps);
                    }
                    "source" => {
                        let mut srcs = Vec::new();
                        if let Item::Value(v) = v {
                            if let Some(arr) = v.as_array() {
                                for a in arr.iter() {
                                    a.as_str().map(|s| srcs.push(s.to_string()));
                                }
                            }
                        }
                        source = Some(srcs);
                    }
                    _ => {
                        return Err(gen_utils::error::Error::Parse(ParseError::new(
                            format!("Invalid key: {}", k).as_str(),
                            ParseType::Toml,
                        )));
                    }
                }
            }
        }

        Ok(PacmanConfig {
            conflicts,
            depends,
            files,
            provides,
            replaces,
            source,
        })
    }
}
