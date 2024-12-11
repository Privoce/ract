use std::{fmt::Display, path::Path, str::FromStr};

use chrono::Datelike;
use gen_utils::{
    common::{fs, ToToml},
    compiler::{Author, License},
    error::Error,
};
use inquire::{Confirm, Select, Text};
use toml_edit::{value, Array, DocumentMut, Item, Table};

use crate::core::entry::{Underlayer, FrameworkType};

/// # Project Info for GenUI project
/// use in ui project.Cargo.toml
/// ## Convert to toml format and write into Cargo.toml file
/// use toml_edit crate to convert to toml format
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<Author>>,
    pub description: Option<String>,
    pub license: License,
    pub keywords: Vec<String>,
    /// underlayer project only when project is gen_ui
    pub underlayer: Option<Underlayer>,
}

impl ProjectInfo {
    pub fn new(is_gen_ui: bool) -> Result<ProjectInfo, Error> {
        let underlayer = if is_gen_ui {
            Some(Underlayer::from_str(
                Select::new(
                    "Which underlayer you want to select?",
                    Underlayer::options(),
                )
                .with_help_message("Now only support Makepad, use enter to skip.")
                .prompt()
                .map_err(|_| Error::from("Failed to get underlayer"))?,
            )?)
        } else {
            None
        };

        let name = Text::new("Project name:")
            .with_placeholder("Your project name use snake_case")
            .prompt()
            .map_err(|_| Error::from("Failed to get project name"))?;

        let authors = Text::new("Authors name:")
            .with_placeholder("format: name <email> and use `,` to separate multiple authors")
            .prompt_skippable()
            .map_err(|_| Error::from("Failed to get authors name"))?
            .filter(|s| !s.is_empty());

        let description = Text::new("Project description:")
            .with_default("This project is created by ract. Repo: https://github.com/Privoce/GenUI")
            .prompt_skippable()
            .map_err(|_| Error::from("Failed to get project description"))?;

        let license = Select::new("Choose LICENSE:", License::options())
            .prompt()
            .map_err(|_| Error::from("Failed to get license"))?;

        let version = Text::new("Version:")
            .with_default("0.1.0")
            .with_placeholder("0.1.0")
            .prompt()
            .map_err(|_| Error::from("Failed to get version"))?;

        let keywords = Text::new("Keywords:")
            .with_help_message("You can input multiple keywords, or press Enter to skip")
            .with_default("front_end, ui")
            .with_placeholder("gen_ui, front_end, ui")
            .prompt()
            .map_err(|_| Error::from("Failed to get keywords"))?;

        if
        // confirm the project information
        Confirm::new("Do you confirm the project information?")
            .with_default(true)
            .with_help_message(
                "If you confirm, the project will be created with the above information",
            )
            .prompt()
            .map_err(|_| Error::from("Failed to confirm project information"))?
        {
            let authors = authors.map(|authors| {
                authors
                    .split(',')
                    .map(|author| author.parse().unwrap())
                    .collect()
            });

            return Ok(ProjectInfo {
                name,
                version,
                authors,
                description,
                license: license.parse().unwrap(),
                keywords: keywords.split(',').map(|x| x.trim().to_string()).collect(),
                underlayer,
            });
        } else {
            return Self::new(is_gen_ui);
        }
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
    pub fn write_gen_ui_toml<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        self.underlayer.unwrap().write_gen_ui_toml(path.as_ref())
    }
}

impl ToToml for ProjectInfo {
    fn to_toml(&self) -> DocumentMut {
        let mut toml = Table::new();
        // [package] -----------------------------------------------------------------------------------------------
        let mut package = Table::new();
        // - [name] ------------------------------------------------------------------------------------------------
        package.insert("name", value(&self.name));
        // - [version] ---------------------------------------------------------------------------------------------
        package.insert("version", value(&self.version));
        // - [authors] ---------------------------------------------------------------------------------------------
        if let Some(authors) = self.authors.as_ref() {
            let authors = authors.iter().fold(Array::new(), |mut arr, author| {
                arr.push(author.to_string());
                arr
            });
            package.insert("authors", value(authors));
        }
        // - [description] -----------------------------------------------------------------------------------------
        if let Some(description) = self.description.as_ref() {
            package.insert("description", value(description));
        }
        // - [license] ---------------------------------------------------------------------------------------------
        package.insert("license", value(self.license.to_string()));
        // - [keywords] --------------------------------------------------------------------------------------------
        let keywords = self.keywords.iter().fold(Array::new(), |mut arr, keyword| {
            arr.push(keyword);
            arr
        });
        package.insert("keywords", value(keywords));

        toml.insert("package", Item::Table(package));
        // [dependencies] -------------------------------------------------------------------------------------------
        // dependencies only add when project is makepad
        if self.underlayer.is_none() {
            match FrameworkType::Makepad.dependencies() {
                Ok(deps) => {
                    toml.insert("dependencies", deps);
                }
                Err(e) => panic!("{}", e.to_string()),
            }
        }else{
            toml.insert("dependencies", Item::None);
        }
        DocumentMut::from(toml)
    }
}

impl Display for ProjectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
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
