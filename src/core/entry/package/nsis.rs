use std::fmt::Display;

use toml_edit::{value, Formatted, Item, Table, Value};

/// # NsisConfig
///
/// The NSIS format configuration.
#[derive(Debug, Clone)]
pub struct NsisConfig {
    /// List of paths where your app stores data.
    /// This options tells the uninstaller to provide the user with an option (disabled by default)
    /// whether they want to rmeove your app data or keep it.
    pub appdata_paths: Option<Vec<String>>,
    /// Set the compression algorithm used to compress files in the installer.
    pub compression: Option<NsisCompression>,
    /// An key-value pair where the key is the language and the value is the path to a custom .nsi file
    /// that holds the translated text for cargo-packager’s custom messages.
    pub custom_language_files: Option<String>,
    /// Whether to display a language selector dialog before the installer
    /// and uninstaller windows are rendered or not.
    /// By default the OS language is selected, with a fallback to the first language in the languages array.
    pub display_language_selector: bool,
    /// The path to a bitmap file to display on the header of installers pages.
    /// The recommended dimensions are 150px x 57px.
    pub header_image: Option<String>,
    /// The path to an icon file used as the installer icon.
    pub installer_icon: Option<String>,
    /// Whether the installation will be for all users or just the current user.
    pub install_mode: Option<NSISInstallerMode>,
    /// A list of installer languages. By default the OS language is used.
    /// If the OS language is not in the list of languages, the first language will be used.
    /// To allow the user to select the language, set display_language_selector to true.
    pub languages: Option<Vec<String>>,
    /// Logic of an NSIS section that will be ran before the install section.
    pub preinstall_section: Option<String>,
    /// The path to a bitmap file for the Welcome page and the Finish page.
    /// The recommended dimensions are 164px x 314px.
    pub sidebar_image: Option<String>,
    /// A custom .nsi template to use.
    pub template: Option<String>,
}

impl Default for NsisConfig {
    fn default() -> Self {
        Self {
            appdata_paths: Some(vec![
                "$APPDATA/$PUBLISHER/$PRODUCTNAME".to_string(),
                "$LOCALAPPDATA/$PRODUCTNAME".to_string(),
            ]),
            compression: Default::default(),
            custom_language_files: Default::default(),
            display_language_selector: Default::default(),
            header_image: Default::default(),
            installer_icon: Default::default(),
            install_mode: Default::default(),
            languages: Default::default(),
            preinstall_section: Default::default(),
            sidebar_image: Default::default(),
            template: Default::default(),
        }
    }
}

impl From<&NsisConfig> for Item {
    fn from(v: &NsisConfig) -> Self {
        let mut table = Table::new();
        if let Some(appdata_paths) = v.appdata_paths.as_ref() {
            let mut arr = toml_edit::Array::default();
            for a in appdata_paths {
                arr.push(a);
            }
            table.insert("appdata-paths", value(arr));
        }
        if let Some(compression) = v.compression.as_ref() {
            table.insert("compression", value(compression));
        }
        if let Some(custom_language_files) = v.custom_language_files.as_ref() {
            table.insert("custom-language-files", value(custom_language_files));
        }
        table.insert(
            "display-language-selector",
            value(v.display_language_selector),
        );
        if let Some(header_image) = v.header_image.as_ref() {
            table.insert("header-image", value(header_image));
        }
        if let Some(installer_icon) = v.installer_icon.as_ref() {
            table.insert("installer-icon", value(installer_icon));
        }
        if let Some(install_mode) = v.install_mode.as_ref() {
            table.insert("install-mode", value(install_mode));
        }
        if let Some(languages) = v.languages.as_ref() {
            let mut arr = toml_edit::Array::default();
            for l in languages {
                arr.push(l);
            }
            table.insert("languages", value(arr));
        }
        if let Some(preinstall_section) = v.preinstall_section.as_ref() {
            table.insert("preinstall-section", value(preinstall_section));
        }
        if let Some(sidebar_image) = v.sidebar_image.as_ref() {
            table.insert("sidebar-image", value(sidebar_image));
        }
        if let Some(template) = v.template.as_ref() {
            table.insert("template", value(template));
        }
        table.set_implicit(false);
        toml_edit::Item::Table(table)
    }
}
impl Display for NsisConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

/// # NSISInstallerMode
///
/// One of the following:
///
/// - "currentUser" Default mode for the installer. Install the app by default in a directory that doesn’t require Administrator access. Installer metadata will be saved under the HKCU registry path.
/// - "perMachine" Install the app by default in the Program Files folder directory requires Administrator access for the installation. Installer metadata will be saved under the HKLM registry path.
/// - "both" Combines both modes and allows the user to choose at install time whether to install for the current user or per machine. Note that this mode will require Administrator access even if the user wants to install it for the current user only. Installer metadata will be saved under the HKLM or HKCU registry path based on the user’s choice.
///
/// Install Modes for the NSIS installer.
#[derive(Debug, Clone, Default)]
pub enum NSISInstallerMode {
    #[default]
    CurrentUser,
    PerMachine,
    Both,
}

impl From<&NSISInstallerMode> for Value {
    fn from(value: &NSISInstallerMode) -> Self {
        Value::String(Formatted::new(
            match value {
                NSISInstallerMode::CurrentUser => "currentUser",
                NSISInstallerMode::PerMachine => "perMachine",
                NSISInstallerMode::Both => "both",
            }
            .to_string(),
        ))
    }
}

impl Display for NSISInstallerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(value(self).to_string().as_str())
    }
}

/// NsisCompression
///
/// One of the following:
///
/// - "zlib" ZLIB uses the deflate algorithm, it is a quick and simple method. With the default compression level it uses about 300 KB of memory.
/// - "bzip2" BZIP2 usually gives better compression ratios than ZLIB, but it is a bit slower and uses more memory. With the default compression level it uses about 4 MB of memory.
/// - "lzma" LZMA (default) is a new compression method that gives very good compression ratios. The decompression speed is high (10-20 MB/s on a 2 GHz CPU), the compression speed is lower. The memory size that will be used for decompression is the dictionary size plus a few KBs, the default is 8 MB.
/// - "off" Disable compression.
///
/// Compression algorithms used in the NSIS installer.
///
/// See <https://nsis.sourceforge.io/Reference/SetCompressor>
#[derive(Debug, Clone, Default)]
pub enum NsisCompression {
    #[default]
    LZMA,
    Zlib,
    Bzip2,
    Off,
}

impl Display for NsisCompression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(value(self).to_string().as_str())
    }
}

impl From<&NsisCompression> for Value {
    fn from(value: &NsisCompression) -> Self {
        Value::String(Formatted::new(
            match value {
                NsisCompression::LZMA => "lzma",
                NsisCompression::Zlib => "zlib",
                NsisCompression::Bzip2 => "bzip2",
                NsisCompression::Off => "off",
            }
            .to_string(),
        ))
    }
}
