use std::fmt::Display;

use toml_edit::{value, Item, Table};

use super::{Position, Size};

/// # DmgConfig
/// The Apple Disk Image (.dmg) configuration.
#[derive(Debug, Clone)]
pub struct DmgConfig {
    /// Position of application folder on window.
    pub app_folder_position: Option<Position>,
    /// Position of application file on window.
    pub app_position: Option<Position>,
    /// Image to use as the background in dmg file. Accepted formats: png/jpg/gif.
    pub background: Option<String>,
    /// Position of volume window on screen.
    pub window_position: Option<Position>,
    /// Size of volume window.
    pub window_size: Option<Size>,
}

impl Default for DmgConfig {
    fn default() -> Self {
        DmgConfig {
            app_folder_position: Some(Position { x: 760, y: 250 }),
            app_position: Some(Position { x: 200, y: 250 }),
            background: Some("./package/dmg_background.png".to_string()),
            window_position: None,
            window_size: Some(Size {
                width: 960,
                height: 540,
            }),
        }
    }
}

impl Display for DmgConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl From<&DmgConfig> for Item {
    fn from(v: &DmgConfig) -> Self {
        let mut table = Table::new();

        if let Some(app_folder_position) = v.app_folder_position.as_ref() {
            table.insert("application-folder-positio", value(app_folder_position));
        }

        if let Some(app_position) = v.app_position.as_ref() {
            table.insert("app-position", value(app_position));
        }

        if let Some(background) = v.background.as_ref() {
            table.insert("background", value(background));
        }

        if let Some(window_position) = v.window_position.as_ref() {
            table.insert("window-position", value(window_position));
        }

        if let Some(window_size) = v.window_size.as_ref() {
            table.insert("window-size", value(window_size));
        }
        table.set_implicit(false);
        toml_edit::Item::Table(table)
    }
}

impl TryFrom<&Item> for DmgConfig {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let mut app_folder_position = None;
        let mut app_position = None;
        let mut background = None;
        let mut window_position = None;
        let mut window_size = None;

        if let Some(table) = value.as_table() {
            for (k, v) in table.iter() {
                match k {
                    "application-folder-positio" => {
                        app_folder_position = Some(Position::try_from(v)?);
                    }
                    "app-position" => {
                        app_position = Some(Position::try_from(v)?);
                    }
                    "background" => {
                        background = Some(v.as_str().unwrap().to_string());
                    }
                    "window-position" => {
                        window_position = Some(Position::try_from(v)?);
                    }
                    "window-size" => {
                        window_size = Some(Size::try_from(v)?);
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

        Ok(DmgConfig {
            app_folder_position,
            app_position,
            background,
            window_position,
            window_size,
        })
    }
}
