use std::{
    env::current_dir,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use gen_utils::common::{fs::path_to_str, ToToml};
use toml_edit::{value, Array, DocumentMut, Table};

use crate::core::log::LogLevel;

use super::{
    AppCategory, Binary, DebianConfig, DmgConfig, FileAssociation, MacOsConfig, NsisConfig,
    PackageFormat, PackageGenerator, PacmanConfig, Position, Resource, Size, WindowsConfig,
    WixConfig,
};

/// cargo packager configuration
pub struct Conf {
    // --- basic -------------------------------------------------------------------
    /// The app name, this is just an identifier that could be used to filter
    /// which app to package using --packages cli arg when there is multiple apps
    /// in the workspace or in the same config.
    pub name: String,
    pub version: String,
    /// The package’s product name, for example “My Awesome App”.
    pub product_name: String,
    /// The application identifier in reverse domain name notation (e.g. com.packager.example).
    /// This string must be unique across applications since it is used in some system configurations.
    /// This string must contain only alphanumeric characters (A–Z, a–z, and 0–9), hyphens (-), and periods (.).
    pub identifier: String,
    /// off is not use in here
    pub log_level: Option<LogLevel>,
    /// The app’s icon list. Supports glob patterns.
    pub icons: Option<Vec<PathBuf>>,
    /// The app’s authors.
    pub authors: Option<Vec<String>>,
    /// The app’s publisher. Defaults to the second element in Config::identifier string.
    /// Currently maps to the Manufacturer property of the Windows Installer.
    pub publisher: Option<String>,
    /// The app’s categories. (macos|debian)
    pub category: Option<AppCategory>,
    pub copyright: Option<String>,
    /// The app’s description.
    pub description: Option<String>,
    /// The app’s long description.
    pub long_description: Option<String>,
    /// The package’s homepage.
    pub homepage: Option<String>,
    /// Whether this config is enabled or not. Defaults to true.
    pub enabled: bool,
    /// A path to the license file.
    pub license_file: Option<PathBuf>,
    // The directory where the [Config::binaries] exist and where the generated packages will be placed.
    pub out_dir: PathBuf,
    // --- platforms ---------------------------------------------------------------
    pub deb: Option<DebianConfig>,
    pub dmg: Option<DmgConfig>,
    pub macos: Option<MacOsConfig>,
    pub nsis: Option<NsisConfig>,
    pub pacman: Option<PacmanConfig>,
    pub windows: Option<WindowsConfig>,
    pub wix: Option<WixConfig>,
    // --- commands ----------------------------------------------------------------
    /// The command to run before packaging each format for an application.
    /// This will run multiple times depending on the formats specifed.
    pub before_each_package_command: Option<String>,
    /// The command to run before starting to package an application.
    /// This runs only once.
    pub before_packaging_command: Option<String>,
    // --- other -------------------------------------------------------------------
    /// The binaries to package
    pub binaries: Vec<Binary>,
    /// Paths to external binaries to add to the package.
    /// The path specified should not include -&lt;target-triple&gt;&lt;.exe&gt; suffix,
    /// it will be auto-added when by the packager when reading these paths, so the actual binary name
    /// should have the target platform’s target triple appended, as well as .exe for Windows.
    /// For example, if you’re packaging an external binary called sqlite3, the packager expects a binary
    /// named sqlite3-x86_64-unknown-linux-gnu on linux, and sqlite3-x86_64-pc-windows-gnu.exe on windows.
    /// If you are building a universal binary for MacOS, the packager expects your external binary to also be universal,
    /// and named after the target triple, e.g. sqlite3-universal-apple-darwin.
    /// See <https://developer.apple.com/documentation/apple-silicon/building-a-universal-macos-binary>
    pub external_binaries: Option<Vec<String>>,
    /// The file associations
    pub file_associations: Option<Vec<FileAssociation>>,
    /// The packaging formats to create, if not present, [PackageFormat::platform_default] is used.
    pub formats: Option<Vec<PackageFormat>>,
    /// The target triple we are packaging for. This mainly affects [Config::external_binaries].
    /// Defaults to the current OS target triple.
    pub target_triple: Option<String>,
    /// The app’s resources to package. This a list of either a glob pattern, path to a file,
    /// path to a directory or an object of src and target paths.
    /// In the case of using an object, the src could be either a glob pattern, path to a file, path to a directory,
    /// and the target is a path inside the final resources folder in the installed package.
    /// Format-specific:
    /// [PackageFormat::Nsis] / [PackageFormat::Wix]: The resources are placed next to the executable in the root of the packager.
    /// [PackageFormat::Deb]: The resources are placed in usr/lib of the package.
    pub resources: Option<Vec<Resource>>,
}

impl ToToml for Conf {
    fn to_toml(&self) -> DocumentMut {
        DocumentMut::from(self.to_toml_table())
    }
}

impl Conf {
    pub fn new(
        name: String,
        version: String,
        product_name: String,
        identifier: String,
        authors: Option<Vec<String>>,
        license: Option<PathBuf>,
    ) -> Self {
        let binaries = vec![Binary {
            main: true,
            path: current_dir()
                .unwrap()
                .join("target")
                .join("release")
                .join(&name),
        }];
        Self {
            name,
            version,
            product_name,
            identifier,
            log_level: None,
            icons: None,
            authors,
            publisher: None,
            category: None,
            copyright: None,
            description: None,
            long_description: None,
            homepage: None,
            enabled: true,
            license_file: license,
            out_dir: PathBuf::from_str("./dist").unwrap(),
            deb: None,
            dmg: None,
            macos: None,
            nsis: None,
            pacman: None,
            windows: None,
            wix: None,
            before_each_package_command: None,
            before_packaging_command: None,
            binaries,
            external_binaries: None,
            file_associations: None,
            formats: None,
            target_triple: None,
            resources: None,
        }
    }

    pub fn makepad<P>(&mut self, path: P) -> PackageGenerator
    where
        P: AsRef<Path>,
    {
        // do makepad configs ---------------------------------------------------------------------
        // [icons] --------------------------------------------------------------------------------
        self.icons = Some(vec![
            PathBuf::from_str("./packaging/app_icon_128.png").unwrap()
        ]);
        // [before_packaging_command|before_each_package_command] ---------------------------------
        let command = BEFORE_COMMAND.replace("${name}", &self.name);
        self.before_each_package_command = Some(command.replace("${cmd}", "before-each-package"));
        self.before_packaging_command = Some(command.replace("${cmd}", "before-packaging"));
        // [resources] -----------------------------------------------------------------------------
        let src_path_pre = PathBuf::from_str(
            format!("{}/resources", path_to_str(self.out_dir.as_path())).as_str(),
        )
        .unwrap();
        let project_name = self.name.to_string();
        self.resources = Some(vec![
            Resource::new_obj(src_path_pre.join("makepad_widgets"), "makepad_widgets"),
            Resource::new_obj(src_path_pre.join(&project_name), &project_name),
        ]);
        // [platforms] -----------------------------------------------------------------------------
        self.deb = Some(DebianConfig {
            depends: Some(vec![format!(
                "{}/depends_deb.txt",
                path_to_str(self.out_dir.as_path())
            )]),
            desktop_template: Some(format!("./packaging/{}.desktop", &self.name)),
            files: None,
            priority: None,
            section: None,
        });

        self.macos = Some(MacOsConfig {
            entitlements: Some("./packaging/Entitlements.plist".to_string()),
            exception_domain: None,
            frameworks: None,
            info_plist_path: Some("./packaging/macos_info.plist".to_string()),
            minimum_system_version: Some("11.0".to_string()),
            provider_short_name: None,
            signing_identity: None,
        });

        self.dmg = Some(DmgConfig {
            app_folder_position: Some(Position { x: 760, y: 250 }),
            app_position: Some(Position { x: 200, y: 250 }),
            background: Some("./packaging/dmg_background.png".to_string()),
            window_position: None,
            window_size: Some(Size {
                width: 960,
                height: 540,
            }),
        });

        let mut nsis = NsisConfig::default();
        nsis.appdata_paths = Some(vec![
            "$APPDATA/$PUBLISHER/$PRODUCTNAME".to_string(),
            "$LOCALAPPDATA/$PRODUCTNAME".to_string(),
        ]);
        self.nsis = Some(nsis);

        self.windows = Some(WindowsConfig::default());
        // generate packing project for makepad ---------------------------------------------------
        PackageGenerator::new(path)
    }

    pub fn to_toml_table(&self) -> Table {
        let mut table = Table::new();
        // [basic] ---------------------------------------------------------------------------------
        table.insert("name", value(&self.name));
        table.insert("version", value(&self.version));
        table.insert("product-name", value(&self.product_name));
        table.insert("identifier", value(&self.identifier));
        if let Some(authors) = self.authors.as_ref() {
            let mut authors_val = Array::new();
            for author in authors {
                authors_val.push(author);
            }
            table.insert("authors", value(authors_val));
        }
        if let Some(license) = self.license_file.as_ref() {
            table.insert("license-file", value(path_to_str(license)));
        }
        table.insert("out-dir", value(path_to_str(&self.out_dir)));
        table.insert("enabled", value(self.enabled));
        if let Some(icons) = self.icons.as_ref() {
            let mut icons_val = Array::new();
            for icon in icons {
                icons_val.push(path_to_str(icon));
            }
            table.insert("icons", value(icons_val));
        }
        if let Some(publisher) = self.publisher.as_ref() {
            table.insert("publisher", value(publisher));
        }
        if let Some(category) = self.category.as_ref() {
            table.insert("category", value(category.to_string()));
        }
        if let Some(description) = self.description.as_ref() {
            table.insert("description", value(description));
        }
        if let Some(long_description) = self.long_description.as_ref() {
            table.insert("long-description", value(long_description));
        }
        if let Some(homepage) = self.homepage.as_ref() {
            table.insert("homepage", value(homepage));
        }
        if let Some(log_level) = self.log_level.as_ref() {
            table.insert("log-level", value(log_level.to_string()));
        }
        if let Some(license) = self.license_file.as_ref() {
            table.insert("license-file", value(path_to_str(license)));
        }
        if let Some(copyright) = self.copyright.as_ref() {
            table.insert("copyright", value(copyright));
        }
        // [platforms] -----------------------------------------------------------------------------
        if let Some(deb) = self.deb.as_ref() {
            table.insert("deb", deb.into());
        }
        if let Some(dmg) = self.dmg.as_ref() {
            table.insert("dmg", dmg.into());
        }
        if let Some(macos) = self.macos.as_ref() {
            table.insert("macos", macos.into());
        }
        if let Some(nsis) = self.nsis.as_ref() {
            table.insert("nsis", nsis.into());
        }
        if let Some(pacman) = self.pacman.as_ref() {
            table.insert("pacman", pacman.into());
        }
        if let Some(windows) = self.windows.as_ref() {
            table.insert("windows", windows.into());
        }
        if let Some(wix) = self.wix.as_ref() {
            table.insert("wix", wix.into());
        }
        // [commands] -------------------------------------------------------------------------------
        if let Some(before_each_package_command) = self.before_each_package_command.as_ref() {
            table.insert(
                "before-each-package-command",
                value(before_each_package_command),
            );
        }
        if let Some(before_packaging_command) = self.before_packaging_command.as_ref() {
            table.insert("before-packaging-command", value(before_packaging_command));
        }
        // [other] ---------------------------------------------------------------------------------
        let mut binaries_val = Array::new();
        for binary in &self.binaries {
            binaries_val.push(binary);
        }
        table.insert("binaries", value(binaries_val));

        if let Some(external_binaries) = self.external_binaries.as_ref() {
            let mut external_binaries_val = Array::new();
            for external_binary in external_binaries {
                external_binaries_val.push(external_binary);
            }
            table.insert("external-binaries", value(external_binaries_val));
        }
        if let Some(file_associations) = self.file_associations.as_ref() {
            let mut file_associations_val = Array::new();
            for file_association in file_associations {
                file_associations_val.push(file_association);
            }
            table.insert("file-associations", value(file_associations_val));
        }
        if let Some(formats) = self.formats.as_ref() {
            let mut formats_val = Array::new();
            for format in formats {
                formats_val.push(format);
            }
            table.insert("formats", value(formats_val));
        }
        if let Some(target_triple) = self.target_triple.as_ref() {
            table.insert("target-triple", value(target_triple));
        }
        if let Some(resources) = self.resources.as_ref() {
            let mut resources_val = Array::new();
            for resource in resources {
                resources_val.push(resource);
            }
            table.insert("resources", value(resources_val));
        }

        table
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

const BEFORE_COMMAND: &str = r#"
cargo run --manifest-path packaging/command/Cargo.toml ${cmd} \
    --force-makepad \
    --binary-name ${name} \
    --path-to-binary ./target/release/${name}
"#;

#[cfg(test)]
mod test_conf {
    use std::str::FromStr;

    use super::Conf;

    #[test]
    fn to_toml() {
        let mut conf = Conf::new(
            "test".to_string(),
            "0.1.0".to_string(),
            "Test".to_string(),
            "com.test".to_string(),
            Some(vec!["test".to_string()]),
            Some(std::path::PathBuf::from_str("./LICENSE").unwrap()),
        );
        let _ = conf.makepad("./test");

        println!("{}", conf.to_string());
    }
}
