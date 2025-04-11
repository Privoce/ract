use std::{
    env::current_dir,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use gen_utils::error::Error;
use gen_utils::{
    common::{fs::path_to_str, ToToml},
    error::ConvertError,
};
use toml_edit::{value, Array, DocumentMut, Table};

use crate::{entry::FrameworkType, log::LogLevel, common::is_workspace};

use super::{
    AppCategory, Binary, DebianConfig, DmgConfig, FileAssociation, MacOsConfig, NsisConfig,
    PackageFormat, PackageGenerator, PacmanConfig, Resource, WindowsConfig, WixConfig,
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
        framework: Option<&FrameworkType>,
    ) -> Self {
        // [binaries] -----------------------------------------------------------------------------
        let current_path = current_dir().unwrap();
        let path = if let Some(FrameworkType::GenUI) = framework {
            current_path
        } else {
            if is_workspace(current_path.as_path()) {
                // get father
                current_path.parent().unwrap().to_path_buf()
            } else {
                current_path
            }
        }
        .join("target")
        .join("release")
        .join(&name);

        let binaries = vec![Binary { main: true, path }];
        // [out dir] -----------------------------------------------------------------------------
        let out_dir = PathBuf::from("./dist");
        // [icons] --------------------------------------------------------------------------------
        let icons = Some(vec![
            PathBuf::from_str("./package/app_icon_128.png").unwrap()
        ]);
        // [platforms] -----------------------------------------------------------------------------
        let deb = Some(DebianConfig {
            depends: Some(vec![format!(
                "{}/depends_deb.txt",
                path_to_str(out_dir.as_path())
            )]),
            desktop_template: Some(format!("./package/{}.desktop", &name)),
            files: None,
            priority: None,
            section: None,
        });
        // [resources] -----------------------------------------------------------------------------
        let resources = Some(vec![Resource::new_obj(
            out_dir.as_path().join("resources").join(&name),
            &name,
        )]);

        Self {
            name,
            version,
            product_name,
            identifier,
            log_level: None,
            icons,
            authors,
            publisher: None,
            category: None,
            copyright: None,
            description: None,
            long_description: None,
            homepage: None,
            enabled: true,
            license_file: license,
            out_dir,
            deb,
            dmg: Some(DmgConfig::default()),
            macos: Some(MacOsConfig::default()),
            nsis: Some(NsisConfig::default()),
            pacman: None,
            windows: Some(WindowsConfig::default()),
            wix: None,
            before_each_package_command: None,
            before_packaging_command: None,
            binaries,
            external_binaries: None,
            file_associations: None,
            formats: None,
            target_triple: None,
            resources,
        }
    }
    pub fn dist_path(&self, framework: Option<&FrameworkType>) -> PathBuf {
        if let Some(FrameworkType::GenUI) = framework {
            current_dir()
                .unwrap()
                .join(self.name.as_str())
                .join(self.out_dir.as_path())
        } else {
            self.out_dir.to_path_buf()
        }
    }
    pub fn path(&self, framework: Option<&FrameworkType>) -> Option<PathBuf> {
        if let Some(FrameworkType::GenUI) = framework {
            Some(PathBuf::from(self.name.as_str()))
        } else {
            None
        }
    }
    pub fn dist_resources(&self, framework: Option<&FrameworkType>) -> PathBuf {
        self.dist_path(framework).join("resources")
    }

    /// ## Generate a package generator for the package
    /// if framework is None, it will generate a package generator for normal package which without additional `rces, before-packaging-command...` items
    pub fn generator<P>(&mut self, path: P, framework: Option<FrameworkType>) -> PackageGenerator
    where
        P: AsRef<Path>,
    {
        match framework {
            Some(f) => match f {
                FrameworkType::GenUI => {
                    // change out_dir to ../
                    self.out_dir = PathBuf::from("../dist");
                    // do not need use makepad_widgets resources (smaller package, about reduce 30MB)
                    self.resources.as_mut().unwrap().push(Resource::new_obj(
                        self.out_dir
                            .as_path()
                            .join("resources")
                            .join("gen_components"),
                        "gen_components",
                    ));
                }
                FrameworkType::Makepad => {
                    self.resources.as_mut().unwrap().push(Resource::new_obj(
                        self.out_dir
                            .as_path()
                            .join("resources")
                            .join("makepad_widgets"),
                        "makepad_widgets",
                    ));
                }
            },
            None => {}
        };
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
        // [platforms] ------------------------------------------------------------------------------
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

    pub fn patch_to_cargo_toml(&self, cargo_toml: &mut DocumentMut) -> () {
        cargo_toml
            .get_mut("package")
            .and_then(|v| v.as_table_mut())
            .map(|v| {
                let mut table = Table::new();
                table.insert("packager", toml_edit::Item::Table(self.to_toml_table()));
                v.insert("metadata", toml_edit::Item::Table(table));
            });
    }

    pub fn from_cargo_toml<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        (&Self::read(&path)?).try_into()
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl TryFrom<&DocumentMut> for Conf {
    type Error = Error;

    fn try_from(doc_mut: &DocumentMut) -> Result<Self, Self::Error> {
        fn get_to_str(table: &Table, key: &str) -> Result<String, Error> {
            table
                .get(key)
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .ok_or_else(|| err_from_to(key, "String"))
        }

        let table = doc_mut["package"]["metadata"]["packager"]
            .as_table()
            .map_or_else(
                || Err(err_from_to("toml [package.metadata.packager]", "Table")),
                |v| Ok(v),
            )?;

        let name = get_to_str(table, "name")?;
        let version = get_to_str(table, "version")?;
        let product_name = get_to_str(table, "product-name")?;
        let identifier = get_to_str(table, "identifier")?;

        let log_level = table
            .get("log-level")
            .and_then(|v| v.as_str().map(|s| LogLevel::from_str(s).unwrap()));

        let icons = table.get("icons").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| i.as_str().map(|s| PathBuf::from(s)))
                    .collect::<Option<Vec<PathBuf>>>()
            })
        });

        let authors = table.get("authors").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| i.as_str().map(|s| s.to_string()))
                    .collect::<Option<Vec<String>>>()
            })
        });

        let publisher = table
            .get("publisher")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let category = table
            .get("category")
            .map(|v| AppCategory::try_from(v))
            .transpose()?;

        let copyright = table
            .get("copyright")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let description = table
            .get("description")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let long_description = table
            .get("long-description")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let homepage = table
            .get("homepage")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let enabled = table
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let license_file = table
            .get("license-file")
            .and_then(|v| v.as_str().map(|s| PathBuf::from(s)));

        let out_dir = table
            .get("out-dir")
            .and_then(|v| v.as_str().map(|s| PathBuf::from(s)))
            .ok_or_else(|| err_from_to("out-dir", "PathBuf"))?;

        let before_each_package_command = table
            .get("before-each-package-command")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let before_packaging_command = table
            .get("before-packaging-command")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let binaries = table.get("binaries").map_or_else(
            || Err(Error::from("can not get binaries in member")),
            |v| {
                v.as_array().map_or_else(
                    || Err(err_from_to("binaries", "Array")),
                    |arr| {
                        let mut binaries = Vec::new();
                        for i in arr.iter() {
                            let binary = Binary::try_from(i)?;
                            binaries.push(binary);
                        }
                        Ok(binaries)
                    },
                )
            },
        )?;

        let external_binaries = table.get("external-binaries").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| i.as_str().map(|s| s.to_string()))
                    .collect::<Option<Vec<String>>>()
            })
        });

        let file_associations = table.get("file-associations").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| FileAssociation::try_from(i))
                    .collect::<Result<Vec<FileAssociation>, Error>>()
                    .ok()
            })
        });

        let formats = table.get("formats").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| PackageFormat::try_from(i))
                    .collect::<Result<Vec<PackageFormat>, Error>>()
                    .ok()
            })
        });

        let target_triple = table
            .get("target-triple")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let resources = table.get("resources").and_then(|v| {
            v.as_array().and_then(|v| {
                v.iter()
                    .map(|i| Resource::try_from(i))
                    .collect::<Result<Vec<Resource>, Error>>()
                    .ok()
            })
        });

        let deb = table
            .get("deb")
            .map(|v| DebianConfig::try_from(v))
            .transpose()?;

        let dmg = table
            .get("dmg")
            .map(|v| DmgConfig::try_from(v))
            .transpose()?;

        let macos = table
            .get("macos")
            .map(|v| MacOsConfig::try_from(v))
            .transpose()?;

        let nsis = table
            .get("nsis")
            .map(|v| NsisConfig::try_from(v))
            .transpose()?;

        let pacman = table
            .get("pacman")
            .map(|v| PacmanConfig::try_from(v))
            .transpose()?;

        let windows = table
            .get("windows")
            .map(|v| WindowsConfig::try_from(v))
            .transpose()?;

        let wix = table
            .get("wix")
            .map(|v| WixConfig::try_from(v))
            .transpose()?;

        Ok(Self {
            name,
            version,
            product_name,
            identifier,
            log_level,
            icons,
            authors,
            publisher,
            category,
            copyright,
            description,
            long_description,
            homepage,
            enabled,
            license_file,
            out_dir,
            deb,
            dmg,
            macos,
            nsis,
            pacman,
            windows,
            wix,
            before_each_package_command,
            before_packaging_command,
            binaries,
            external_binaries,
            file_associations,
            formats,
            target_triple,
            resources,
        })
    }
}

fn err_from_to(from: &str, to: &str) -> Error {
    Error::Convert(ConvertError::FromTo {
        from: from.to_string(),
        to: to.to_string(),
    })
}
