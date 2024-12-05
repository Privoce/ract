use std::{fmt::Display, path::PathBuf};

use gen_utils::common::fs::path_to_str;
use toml_edit::{value, Array, Formatted, InlineTable, Item, Table, Value};

/// # WixConfig
///
/// The wix format configuration
#[derive(Debug, Clone, Default)]
pub struct WixConfig {
    pub banner_path: Option<String>,
    pub component_group_refs: Option<Vec<String>>,
    pub component_refs: Option<Vec<String>>,
    pub custom_action_refs: Option<Vec<String>>,
    pub dialog_image_path: Option<String>,
    pub feature_group_refs: Option<Vec<String>>,
    pub feature_refs: Option<Vec<String>>,
    pub fips_compliant: bool,
    pub fragment_paths: Option<Vec<String>>,
    pub fragments: Option<Vec<String>>,
    pub languages: Option<Vec<WixLanguage>>,
    pub merge_modules: Option<Vec<String>>,
    pub merge_refs: Option<Vec<String>>,
    pub template: Option<String>,
}

impl From<&WixConfig> for Item {
    fn from(v: &WixConfig) -> Self {
        let mut table = Table::new();

        if let Some(banner_path) = v.banner_path.as_ref() {
            table.insert("banner-path", value(banner_path));
        }
        if let Some(component_group_refs) = v.component_group_refs.as_ref() {
            let mut arr = Array::default();
            for c in component_group_refs {
                arr.push(c);
            }
            table.insert("component-group-refs", value(arr));
        }
        if let Some(component_refs) = v.component_refs.as_ref() {
            let mut arr = Array::default();
            for c in component_refs {
                arr.push(c);
            }
            table.insert("component-refs", value(arr));
        }
        if let Some(custom_action_refs) = v.custom_action_refs.as_ref() {
            let mut arr = Array::default();
            for c in custom_action_refs {
                arr.push(c);
            }
            table.insert("custom-action-refs", value(arr));
        }
        if let Some(dialog_image_path) = v.dialog_image_path.as_ref() {
            table.insert("dialog-image-path", value(dialog_image_path));
        }
        if let Some(feature_group_refs) = v.feature_group_refs.as_ref() {
            let mut arr = Array::default();
            for f in feature_group_refs {
                arr.push(f);
            }
            table.insert("feature-group-refs", value(arr));
        }
        if let Some(feature_refs) = v.feature_refs.as_ref() {
            let mut arr = Array::default();
            for f in feature_refs {
                arr.push(f);
            }
            table.insert("feature-refs", value(arr));
        }
        table.insert("fips-compliant", value(v.fips_compliant));
        if let Some(fragment_paths) = v.fragment_paths.as_ref() {
            let mut arr = Array::default();
            for f in fragment_paths {
                arr.push(f);
            }
            table.insert("fragment-paths", value(arr));
        }
        if let Some(fragments) = v.fragments.as_ref() {
            let mut arr = Array::default();
            for f in fragments {
                arr.push(f);
            }
            table.insert("fragments", value(arr));
        }
        if let Some(languages) = v.languages.as_ref() {
            let mut arr = Array::new();
            for l in languages {
                arr.push(l);
            }
            table.insert("languages", value(arr));
        }

        if let Some(merge_modules) = v.merge_modules.as_ref() {
            let mut arr = Array::default();
            for m in merge_modules {
                arr.push(m);
            }
            table.insert("merge-modules", value(arr));
        }

        if let Some(merge_refs) = v.merge_refs.as_ref() {
            let mut arr = Array::default();
            for m in merge_refs {
                arr.push(m);
            }
            table.insert("merge-refs", value(arr));
        }

        if let Some(template) = v.template.as_ref() {
            table.insert("template", value(template));
        }
        table.set_implicit(false);
        Item::Table(table)
    }
}

impl Display for WixConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

/// # WixLanguage
#[derive(Debug, Clone)]
pub enum WixLanguage {
    String(String),
    Obj {
        identifier: String,
        /// path to .wxl file
        path: Option<PathBuf>,
    },
}

impl Display for WixLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&WixLanguage> for Value {
    fn from(lang: &WixLanguage) -> Self {
        match lang {
            WixLanguage::String(s) => Value::String(Formatted::new(s.to_string())),
            WixLanguage::Obj { identifier, path } => {
                let mut v = InlineTable::new();

                v.insert(
                    "identifier",
                    Value::String(Formatted::new(identifier.to_string())),
                );
                if let Some(p) = path {
                    v.insert(
                        "path",
                        Value::String(Formatted::new(path_to_str(p))),
                    );
                }

                Value::InlineTable(v)
            }
        }
    }
}
