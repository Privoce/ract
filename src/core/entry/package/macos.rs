use std::{collections::HashMap, fmt::Display};

use toml_edit::{value, Table};

use super::PackageConf;

/// # MacOsConfig
///
/// The macOS configuration.
#[derive(Debug, Clone)]
pub struct MacOsConfig {
    /// Path to the entitlements.plist file.
    pub entitlements: Option<String>,
    /// The exception domain to use on the macOS .app package.
    /// This allows communication to the outside world e.g. a web server you’re shipping.
    pub exception_domain: Option<String>,
    /// MacOS frameworks that need to be packaged with the app.
    pub frameworks: Option<Vec<String>>,
    /// Path to the Info.plist file for the package.
    pub info_plist_path: Option<String>,
    /// A version string indicating the minimum MacOS version that the packaged app supports.
    /// If you are using this config field, you may also want have your build.rs script emit
    /// cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11. (e.g. "10.11").
    pub minimum_system_version: Option<String>,
    /// Provider short name for notarization.
    pub provider_short_name: Option<String>,
    /// Code signing identity.
    pub signing_identity: Option<String>,
}

impl Display for MacOsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml_table().to_string().as_str())
    }
}

impl MacOsConfig {
    pub fn to_toml_table(&self) -> Table {
        let mut table = Table::new();
        if let Some(entitlements) = self.entitlements.as_ref() {
            table.insert("entitlements", value(entitlements));
        }
        if let Some(exception_domain) = self.exception_domain.as_ref() {
            table.insert("exception-domain", value(exception_domain));
        }
        if let Some(frameworks) = self.frameworks.as_ref() {
            let mut arr = toml_edit::Array::default();
            for f in frameworks {
                arr.push(f);
            }
            table.insert("frameworks", value(arr));
        }
        if let Some(info_plist_path) = self.info_plist_path.as_ref() {
            table.insert("info-plist-path", value(info_plist_path));
        }
        if let Some(minimum_system_version) = self.minimum_system_version.as_ref() {
            table.insert("minimum-system-version", value(minimum_system_version));
        }
        if let Some(provider_short_name) = self.provider_short_name.as_ref() {
            table.insert("provider-short-name", value(provider_short_name));
        }
        if let Some(signing_identity) = self.signing_identity.as_ref() {
            table.insert("signing-identity", value(signing_identity));
        }
        table.set_implicit(false);
        table
    }

    /// the entitlements file for macos
    /// see: https://developer.apple.com/documentation/bundleresources/entitlements
    pub fn to_entitlements() -> String {
        DEFAULT_PLIST_FMT.replace("${dicts}", "")
    }

    pub fn to_info_plist(conf: &PackageConf) -> String {
        let mut dicts = HashMap::new();
        // get bundle configurations from the package configuration
        // [Bundle] -----------------------------------------------------------------------
        dicts.insert("CFBundleIdentifier", val("string", Some(&conf.identifier)));
        dicts.insert("CFBundleName", val("string", Some(&conf.product_name)));
        dicts.insert(
            "CFBundleDisplayName",
            val("string", Some(&conf.product_name)),
        );
        // all descriptions
        let desc = conf
            .description
            .as_ref()
            .map(|desc| val("string", Some(desc)))
            .unwrap_or(String::new());
        dicts.insert(
            "NSLocationAlwaysAndWhenInUseUsageDescription",
            desc.to_string(),
        );
        dicts.insert("NSLocationAlwaysUsageDescription", desc.to_string());
        dicts.insert("NSLocationWhenInUseUsageDescription", desc.to_string());
        // others
        dicts.insert("CFBundleExecutable", val("string", Some(&conf.name)));
        dicts.insert("CFBundlePackageType", val("string", Some("APPL")));
        dicts.insert("CFBundleInfoDictionaryVersion", val("string", Some("6.0")));
        dicts.insert(
            "CFBundleSpokenName",
            val("string", Some(&conf.product_name)),
        );
        // version for app
        dicts.insert("CFBundleVersion", val("string", Some(&conf.version)));
        dicts.insert(
            "CFBundleShortVersionString",
            val("string", Some(&conf.version)),
        );
        // copyright
        if let Some(copyright) = conf.copyright.as_ref() {
            dicts.insert("NSHumanReadableCopyright", val("string", Some(copyright)));
        }
        if let Some(macos) = conf.macos.as_ref() {
            // LSMinimumSystemVersionByArchitecture, MinimumOSVersion, LSMinimumSystemVersion is the same
            if let Some(minimum_system_version) = macos.minimum_system_version.as_ref() {
                let version = val("string", Some(minimum_system_version));
                dicts.insert("LSMinimumSystemVersion", version.to_string());
                dicts.insert("MinimumOSVersion", version.to_string());
                dicts.insert("LSMinimumSystemVersionByArchitecture", version);
            }
        }
        // now not support watchOS, tvOS, iOS
        dicts.insert("WKWatchKitApp", val("false", None));
        dicts.insert("CFBundleDevelopmentRegion", val("string", Some("en-US")));
        // icons
        dicts.insert(
            "CFBundleIconFile",
            val(
                "string",
                Some(format!("{}.icns", &conf.product_name).as_str()),
            ),
        );
        // about optimization
        dicts.insert("NSHighResolutionCapable", val("true", None));
        dicts.insert("LSRequiresCarbon", val("true", None));
        dicts.insert("NSLocationDefaultAccuracyReduced", val("false", None));
        dicts.insert("CSResourcesFileMapped", val("true", None));

        let dicts = dicts.iter().fold(String::new(), |mut acc, (k, v)| {
            acc.push_str(&format!("<key>{}</key>\n{}", k, v));
            acc
        });

        DEFAULT_PLIST_FMT.replace("${dicts}", &dicts)
    }
}

const DEFAULT_PLIST_FMT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>${dicts}</dict>
</plist>
"#;

fn val(ty: &str, v: Option<&str>) -> String {
    match v {
        Some(v) => format!("<{}>{}</{}>", ty, v, ty),
        None => format!("<{}/>", ty),
    }
}
