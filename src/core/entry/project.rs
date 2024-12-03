use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::Datelike;
use gen_utils::{
    common::{fs, DepType, RustDependence},
    compiler::{Author, License},
    error::{Error, ParseError, ParseType},
};
use toml_edit::{value, Array, DocumentMut};

use crate::core::env::real_chain_env_toml;

/// # Project Info for GenUI project
/// use in ui project.Cargo.toml
/// ## Convert to toml format and write into Cargo.toml file
/// use toml_edit crate to convert to toml format
#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<Author>>,
    pub description: Option<String>,
    pub license: License,
    pub keywords: Vec<String>,
}

impl ProjectInfo {
    pub fn write_gen_ui_cargo_toml<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join("Cargo.toml");
        let mut toml = Self::get_toml_content(path.as_path())?;
        // [write project info] -----------------------------------------------------------------------------------
        let _ = self.write_project_info(&mut toml);
        // write to Cargo.toml file
        fs::write(path.as_path(), &toml.to_string())?;
        Ok(())
    }
    pub fn write_makepad_cargo_toml<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().join("Cargo.toml");
        let mut toml = Self::get_toml_content(path.as_path())?;
        // [write project info] -----------------------------------------------------------------------------------
        let _ = self.write_project_info(&mut toml);
        // [write dependencies] -----------------------------------------------------------------------------------
        // read dependencies from gpiler chain
        let env_toml = real_chain_env_toml()?;
        let mut makepad_path = PathBuf::from_str(
            env_toml["dependencies"]["makepad-widgets"]
                .as_str()
                .unwrap(),
        )
        .map_err(|e| Error::from(e.to_string()))?;
        makepad_path = makepad_path.join("widgets");
        let mut rust_dep = RustDependence::new("makepad-widgets");
        let _ = rust_dep.set_ty(DepType::local(makepad_path));
        let (key, value) = rust_dep.to_table_kv();
        let _ = toml["dependencies"].as_table_mut().map_or_else(
            || Err(Error::from("can not convert to toml table".to_string())),
            |table| {
                table[&key] = value;
                Ok(())
            },
        )?;
        // write to Cargo.toml file
        fs::write(path.as_path(), &toml.to_string())?;
        Ok(())
    }
    fn get_toml_content<P>(path: P) -> Result<DocumentMut, Error>
    where
        P: AsRef<Path>,
    {
        let content = fs::read(path.as_ref())?;
        content
            .parse::<DocumentMut>()
            .map_err(|e| Error::Parse(ParseError::new(e.to_string().as_str(), ParseType::Toml)))
    }

    fn write_project_info(&self, toml: &mut DocumentMut) -> () {
        // write info to [package] section (except name)
        // [version] ----------------------------------------------------------------------------------------------
        toml["package"]["version"] = value(&self.version);
        // [authors] ----------------------------------------------------------------------------------------------
        if let Some(authors) = &self.authors {
            let mut authors_value = Array::new();
            for author in authors {
                authors_value.push(author.to_string());
            }
            toml["package"]["authors"] = value(authors_value);
        }
        // [description] ------------------------------------------------------------------------------------------
        if let Some(description) = &self.description {
            toml["package"]["description"] = value(description);
        }
        // [license] ----------------------------------------------------------------------------------------------
        toml["package"]["license"] = value(self.license.to_string());
        // [keywords] ---------------------------------------------------------------------------------------------
        let mut keywords_value = Array::new();
        for keyword in &self.keywords {
            keywords_value.push(keyword);
        }
        toml["package"]["keywords"] = value(keywords_value);
    }
    pub fn write_license<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let year = chrono::Local::now().year();
        let holder = self
            .authors
            .as_ref()
            .map(|authors| {
                authors
                    .iter()
                    .map(|author| author.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_default();

        if let Some(content) = self.license.content(year, &holder) {
            let _ = fs::write(path.as_ref().join("LICENSE"), &content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod toml_test {
    use std::{path::PathBuf, str::FromStr};

    use gen_utils::common::fs;
    use toml_edit::{value, DocumentMut};

    #[test]
    fn read() {
        let path =
            PathBuf::from_str("/Users/shengyifei/projects/gen_ui/GenUI/examples/hh/hh/Cargo.toml")
                .unwrap();

        let content = fs::read(path.as_path()).unwrap();
        let mut toml = content
            .parse::<DocumentMut>()
            .expect("parse Cargo.toml failed");

        // dbg!(&toml);
        toml["package"]["version"] = value("0.2.0");
        dbg!(&toml["package"]["version"]);
    }
}
