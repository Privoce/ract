use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use gen_utils::{
    common::{
        fs::{self, FileState},
        ToToml,
    },
    compiler::Version,
    error::{Error, ParseError, ParseType},
};

use sha2::{Digest, Sha256};
use toml_edit::{value, DocumentMut, Item, Table};

use crate::log::compiler::CompilerLogs;

/// ## Gen compile cache
/// use msgpack to serialize and deserialize
#[derive(Clone, Debug)]
pub struct Cache {
    /// version for the cache
    version: String,
    /// cache values, key is file path, value is file hash value
    values: HashMap<PathBuf, String>,
}

impl ToToml for Cache {
    fn to_toml(&self) -> toml_edit::DocumentMut {
        let mut toml = Table::new();

        toml.insert("version", value(&self.version));
        let mut values_table = Table::new();
        for (k, v) in self.values.iter() {
            values_table.insert(&fs::path_to_str(k), value(v));
        }
        toml.insert("values", Item::Table(values_table));
        DocumentMut::from(toml)
    }
    fn write<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join(".gen_ui_cache");
        fs::write(path, &self.to_string())
    }
    fn read<P>(path: P) -> Result<DocumentMut, Error>
    where
        P: AsRef<Path>,
    {
        fs::read(path.as_ref().join(".gen_ui_cache")).and_then(|content| {
            content
                .parse::<DocumentMut>()
                .map_err(|e| Error::Parse(ParseError::new(e.to_string().as_str(), ParseType::Toml)))
        })
    }
}

impl Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl TryFrom<&DocumentMut> for Cache {
    type Error = Error;

    fn try_from(value: &DocumentMut) -> Result<Self, Self::Error> {
        let version = value.get("version").map_or_else(
            || {
                Err(
                    ParseError::new("can not get item item in .gen_ui_cache", ParseType::Toml)
                        .into(),
                )
            },
            |version| {
                version.as_str().map_or_else(
                    || {
                        Err(ParseError::new(
                            "can not covert version item",
                            ParseType::Toml,
                        ))
                    },
                    |version| Ok(version.to_string()),
                )
            },
        )?;

        let values = value.get("values").map_or_else(
            || {
                Err(
                    ParseError::new("can not get values item in .gen_ui_cache", ParseType::Toml)
                        .into(),
                )
            },
            |values| {
                values.as_table().map_or_else(
                    || {
                        Err(ParseError::new(
                            "can not covert values item",
                            ParseType::Toml,
                        ))
                    },
                    |values| {
                        let mut values_map = HashMap::new();
                        for (k, v) in values.iter() {
                            values_map.insert(
                                PathBuf::from(k),
                                v.as_str().map_or_else(
                                    || {
                                        Err(ParseError::new(
                                            "can not covert values item",
                                            ParseType::Toml,
                                        ))
                                    },
                                    |v| Ok(v.to_string()),
                                )?,
                            );
                        }
                        Ok(values_map)
                    },
                )
            },
        )?;

        Ok(Self { version, values })
    }
}

impl Cache {
    /// load cache from the path
    /// if the cache file is not exists, create a new empty cache file
    /// if exists, load the cache file
    pub fn new<P>(path: P) -> Result<Cache, Error>
    where
        P: AsRef<Path>,
    {
        let mut is_load = false;
        let cache = match Cache::read(path.as_ref()) {
            Ok(cache) => {
                if let Ok(cache) = (&cache).try_into() {
                    is_load = true;
                    cache
                } else {
                    Cache::default()
                }
            }
            Err(_) => Cache::default(),
        };

        if !is_load {
            // write the cache file
            cache.write(path.as_ref())?;
            CompilerLogs::WriteCache.compiler().info();
        }
        Ok(cache)
    }

    /// if exists, then calc hash with origin, if hash equal, don't insert and return FileState::Unchanged
    ///
    /// if not exists, insert and return FileState::Created
    ///
    /// if exists but hash not equal, insert and return FileState::Modified
    pub fn exists_or_insert<P>(&mut self, key: P) -> Result<FileState, Error>
    where
        P: AsRef<Path>,
    {
        let hash = calc_hash(key.as_ref()).map_err(|e| Error::from(e.to_string()))?;
        if let Some(value) = self.values.get(key.as_ref()) {
            // exist cache
            if value.eq(&hash) {
                return Ok(FileState::Unchanged);
            } else {
                self.insert(key, hash);
                return Ok(FileState::Modified);
            }
        } else {
            self.insert(key, hash);
            return Ok(FileState::Created);
        }
    }
    pub fn insert<P>(&mut self, key: P, value: String) -> ()
    where
        P: AsRef<Path>,
    {
        self.values.insert(key.as_ref().to_path_buf(), value);
    }
    /// clear the cache `[value]` section and write back to the file
    pub fn clear<P>(&mut self, path: P) -> Result<(), Error> where P: AsRef<Path> {
        self.values.clear();
        self.write(path)
    }

    pub fn remove<P>(&mut self, key: P) -> ()
    where
        P: AsRef<Path>,
    {
        self.values.remove(key.as_ref());
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            version: Version::new(0, 0, 2).to_string(),
            values: Default::default(),
        }
    }
}

/// ## calculate the hash of a file
/// calc hash use sha256
pub fn calc_hash<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::options().read(true).open(path.as_ref())?;
    let mut hasher = Sha256::new();

    let mut buf = [0; 1024];
    // loop read
    loop {
        let b_read = file.read(&mut buf)?;
        if b_read == 0 {
            break;
        }
        // update the hasher with the read buffer
        hasher.update(&buf[..b_read]);
    }

    // calc hash
    let hash_value = hasher.finalize();

    Ok(format!("{:x}", hash_value))
}
