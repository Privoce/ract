use std::fmt::Display;
use toml_edit::{value, Item, Table};

/// # WindowsConfig
///
/// The Windows configuration.
#[derive(Debug, Clone)]
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

impl Default for WindowsConfig {
    fn default() -> Self {
        Self {
            allow_downgrades: true,
            certificate_thumbprint: Default::default(),
            digest_algorithm: Default::default(),
            sign_command: Default::default(),
            timestamp_url: Default::default(),
            tsp: Default::default(),
        }
    }
}

impl Display for WindowsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl From<&WindowsConfig> for Item {
    fn from(v: &WindowsConfig) -> Self {
        let mut table = Table::new();
        table.insert("allow-downgrades", value(v.allow_downgrades));
        if let Some(certificate_thumbprint) = v.certificate_thumbprint.as_ref() {
            table.insert("certificate-thumbprint", value(certificate_thumbprint));
        }
        if let Some(digest_algorithm) = v.digest_algorithm.as_ref() {
            table.insert("digest-algorithm", value(digest_algorithm));
        }
        if let Some(sign_command) = v.sign_command.as_ref() {
            table.insert("sign-command", value(sign_command));
        }
        if let Some(timestamp_url) = v.timestamp_url.as_ref() {
            table.insert("timestamp-url", value(timestamp_url));
        }
        table.insert("tsp", value(v.tsp));
        table.set_implicit(false);
        Item::Table(table)
    }
}

impl TryFrom<&Item> for WindowsConfig {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let mut allow_downgrades = true;
        let mut certificate_thumbprint = None;
        let mut digest_algorithm = None;
        let mut sign_command = None;
        let mut timestamp_url = None;
        let mut tsp = false;

        if let Item::Table(table) = value {
            for (k, v) in table.iter() {
                match k {
                    "allow-downgrades" => {
                        allow_downgrades = v.as_bool().map_or(true, |v| v);
                    }
                    "certificate-thumbprint" => {
                        certificate_thumbprint = v.as_str().map(|v| v.to_string());
                    }
                    "digest-algorithm" => {
                        digest_algorithm = v.as_str().map(|v| v.to_string());
                    }
                    "sign-command" => {
                        sign_command = v.as_str().map(|v| v.to_string());
                    }
                    "timestamp-url" => {
                        timestamp_url = v.as_str().map(|v| v.to_string());
                    }
                    "tsp" => {
                        tsp = v.as_bool().map_or(false, |v| v);
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

        Ok(WindowsConfig {
            allow_downgrades,
            certificate_thumbprint,
            digest_algorithm,
            sign_command,
            timestamp_url,
            tsp,
        })
    }
}
