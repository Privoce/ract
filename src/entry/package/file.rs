use std::fmt::Display;

use super::BundleTypeRole;
use gen_utils::error::{ConvertError, Error};
use toml_edit::{value, Array, Formatted, InlineTable, Value};

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

impl TryFrom<&Value> for FileAssociation {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let table = value.as_inline_table().ok_or_else(|| {
            Error::Convert(gen_utils::error::ConvertError::FromTo {
                from: "&toml_edit::Value".to_string(),
                to: "FileAssociation".to_string(),
            })
        })?;

        let description = table
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let extensions = table.get("ext").map_or_else(
            || Err(Error::from("ext is required")),
            |v| {
                v.as_array().map_or_else(
                    || {
                        Err(Error::Convert(ConvertError::FromTo {
                            from: "&toml_edit::Value".to_string(),
                            to: "Array".to_string(),
                        }))
                    },
                    |arr| {
                        let mut exts = Vec::new();
                        for ext in arr.iter() {
                            ext.as_str().map(|s| exts.push(s.to_string()));
                        }
                        Ok(exts)
                    },
                )
            },
        )?;

        let mime_type = table
            .get("mime-type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let name = table
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let role = table.get("role").map_or_else(
            || Err(Error::from("can not get role in file association")),
            |v| BundleTypeRole::try_from(v),
        )?;

        Ok(Self {
            description,
            extensions,
            mime_type,
            name,
            role,
        })
    }
}
