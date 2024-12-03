use std::fmt::Display;

use toml_edit::{value, Array, Table};

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
        f.write_str(self.to_toml_table().to_string().as_str())
    }
}

impl DebianConfig {
    pub fn to_toml_table(&self) -> Table {
        let mut table = Table::new();
        if let Some(depends) = self.depends.as_ref() {
            let mut arr = Array::new();
            for d in depends {
                arr.push(d);
            }
            table.insert("depends", value(arr));
        }

        if let Some(desktop_template) = self.desktop_template.as_ref() {
            table.insert("desktop-template", value(desktop_template));
        }

        if let Some(files) = self.files.as_ref() {
            table.insert("files", value(files));
        }

        if let Some(priority) = self.priority.as_ref() {
            table.insert("priority", value(priority));
        }

        if let Some(section) = self.section.as_ref() {
            table.insert("section", value(section));
        }
        table.set_implicit(false);
        table
    }
}
