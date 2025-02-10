use std::fmt::Display;

use toml_edit::{value, Array, Item, Table};

/// The Linux debian configuration.
pub struct DebianConfig {
    pub depends: Option<Vec<String>>,
    pub desktop_template: Option<String>,
    pub files: Option<String>,
    pub priority: Option<String>,
    pub section: Option<String>,
}

impl Display for DebianConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl From<&DebianConfig> for Item {
    fn from(v: &DebianConfig) -> Self {
        let mut table = Table::new();
        if let Some(depends) = v.depends.as_ref() {
            let mut arr = Array::new();
            for d in depends {
                arr.push(d);
            }
            table.insert("depends", value(arr));
        }

        if let Some(desktop_template) = v.desktop_template.as_ref() {
            table.insert("desktop-template", value(desktop_template));
        }

        if let Some(files) = v.files.as_ref() {
            table.insert("files", value(files));
        }

        if let Some(priority) = v.priority.as_ref() {
            table.insert("priority", value(priority));
        }

        if let Some(section) = v.section.as_ref() {
            table.insert("section", value(section));
        }
        table.set_implicit(false);
        Item::Table(table)
    }
}

impl TryFrom<&Item> for DebianConfig {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let mut depends = None;
        let mut desktop_template = None;
        let mut files = None;
        let mut priority = None;
        let mut section = None;

        if let Item::Table(table) = value {
            for (k, v) in table.iter() {
                match k {
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
                    "desktop-template" => {
                        if let Item::Value(v) = v {
                            desktop_template = v.as_str().map(|s| s.to_string());
                        }
                    }
                    "files" => {
                        if let Item::Value(v) = v {
                            files = v.as_str().map(|s| s.to_string());
                        }
                    }
                    "priority" => {
                        if let Item::Value(v) = v {
                            priority = v.as_str().map(|s| s.to_string());
                        }
                    }
                    "section" => {
                        if let Item::Value(v) = v {
                            section = v.as_str().map(|s| s.to_string());
                        }
                    }
                    _ => {
                        return Err(gen_utils::error::Error::Parse(
                            gen_utils::error::ParseError::new(
                                format!("Invalid key: {}", k).as_str(),
                                gen_utils::error::ParseType::Toml,
                            ),
                        ));
                    }
                }
            }
        }

        Ok(DebianConfig {
            depends,
            desktop_template,
            files,
            priority,
            section,
        })
    }
}
