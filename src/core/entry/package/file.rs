use std::fmt::Display;

use toml_edit::{value, Array, Formatted, InlineTable, Value};

use super::BundleTypeRole;

/// # A file association configuration
pub struct FileAssociation {
    /// The association description. Windows-only. It is displayed on the Type column on Windows Explorer.
    description: Option<String>,
    /// File extensions to associate with this app. e.g. ‘png’ (required)
    extensions: Vec<String>,
    /// The mime-type e.g. ‘image/png’ or ‘text/plain’. Linux-only.
    mime_type: Option<String>,
    /// The name. Maps to CFBundleTypeName on macOS. Defaults to the first item in ext
    name: Option<String>,
    /// The app’s role with respect to the type. Maps to CFBundleTypeRole on macOS. Defaults to [BundleTypeRole::Editor]
    role: BundleTypeRole,
}

impl Display for FileAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(value(self).to_string().as_str())
    }
}

impl From<&FileAssociation> for Value {
    fn from(v: &FileAssociation) -> Self {
        let mut table = InlineTable::new();

        if let Some(description) = v.description.as_ref() {
            table.insert(
                "description",
                Value::String(Formatted::new(description.to_string())),
            );
        }

        let mut exts = Array::new();
        for ext in &v.extensions {
            exts.push(Value::String(Formatted::new(ext.to_string())));
        }
        table.insert("ext", Value::Array(exts));

        if let Some(mime_type) = v.mime_type.as_ref() {
            table.insert(
                "mime-type",
                Value::String(Formatted::new(mime_type.to_string())),
            );
        }

        if let Some(name) = v.name.as_ref() {
            table.insert("name", Value::String(Formatted::new(name.to_string())));
        }

        table.insert("role", Value::String(Formatted::new(v.role.to_string())));
        Value::InlineTable(table)
    }
}
