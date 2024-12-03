use std::fmt::Display;

use toml_edit::{value, Table};

/// # WindowsConfig
///
/// The Windows configuration.
#[derive(Debug, Clone, Default)]
pub struct WindowsConfig {
    /// Whether to validate a second app installation, blocking the user from installing an older version if set to false.
    /// For instance, if 1.2.1 is installed, the user won’t be able to install app version 1.2.0 or 1.1.5.
    /// The default value of this flag is true.
    pub allow_downgrades: bool,
    /// The SHA1 hash of the signing certificate.
    pub certificate_thumbprint: Option<String>,
    /// The file digest algorithm to use for creating file signatures. Required for code signing. SHA-256 is recommended.
    pub digest_algorithm: Option<String>,
    /// Specify a custom command to sign the binaries.
    /// This command needs to have a %1 in it which is just a placeholder for the binary path,
    /// which we will detect and replace before calling the command.
    /// By Default we use signtool.exe which can be found only on Windows
    /// so if you are on another platform and want to cross-compile and sign you will need to use another tool like osslsigncode.
    pub sign_command: Option<String>,
    /// Server to use during timestamping.
    pub timestamp_url: Option<String>,
    /// Whether to use Time-Stamp Protocol (TSP, a.k.a. RFC 3161) for the timestamp server.
    /// Your code signing provider may use a TSP timestamp server, like e.g. SSL.com does.
    /// If so, enable TSP by setting to true.
    pub tsp: bool,
}

impl Display for WindowsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml_table().to_string().as_str())
    }
}

impl WindowsConfig {
    pub fn to_toml_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("allow-downgrades", value(self.allow_downgrades));
        if let Some(certificate_thumbprint) = self.certificate_thumbprint.as_ref() {
            table.insert("certificate-thumbprint", value(certificate_thumbprint));
        }
        if let Some(digest_algorithm) = self.digest_algorithm.as_ref() {
            table.insert("digest-algorithm", value(digest_algorithm));
        }
        if let Some(sign_command) = self.sign_command.as_ref() {
            table.insert("sign-command", value(sign_command));
        }
        if let Some(timestamp_url) = self.timestamp_url.as_ref() {
            table.insert("timestamp-url", value(timestamp_url));
        }
        table.insert("tsp", value(self.tsp));
        table.set_implicit(false);
        table
    }
}
