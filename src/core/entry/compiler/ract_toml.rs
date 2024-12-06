use std::{fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::{
    common::{fs, ToToml},
    error::{Error, ParseError, ParseType},
};
use toml_edit::{value, Array, DocumentMut, Formatted, InlineTable, Value};

use crate::core::entry::{FrameworkType, ProjectInfo};

/// # RactToml
/// each project has a .ract file to point the project kind and help ract to compile the project
///
/// **try support makepad and gen_ui**
/// ## Example
/// ```toml
/// target = "gen_ui"
/// members = [
///    { source = "./hello", target = "./hello_makepad" },
/// ]
/// compiles = [0]
/// ```
#[derive(Debug, Clone)]
pub struct RactToml {
    /// target of the project
    pub target: FrameworkType,
    /// members of the project
    pub members: Option<Vec<Member>>,
    /// projects to compile, if not set, compile the first project in the members
    /// - if compiles length is 0, not compile any project
    /// - if compiles length is 1, compile the project in the members by index
    /// - if compiles length is more than 1, use multiple threads to compile the projects
    pub compiles: Option<Vec<usize>>,
}

impl RactToml {
    /// ## makepad project
    /// if target is makepad, members and compiles must be None
    /// do not need to care about members and compiles
    pub fn makepad() -> Self {
        Self::new(FrameworkType::Makepad, None, None)
    }
    /// gen_ui must be a workspace
    pub fn gen_ui(members: Vec<Member>) -> Self {
        Self::new(FrameworkType::GenUI, Some(members), None)
    }
    pub fn new(
        target: FrameworkType,
        members: Option<Vec<Member>>,
        compiles: Option<Vec<usize>>,
    ) -> Self {
        Self {
            target,
            members,
            compiles,
        }
    }
}

impl ToToml for RactToml {
    fn to_toml(&self) -> toml_edit::DocumentMut {
        let mut doc = DocumentMut::new();

        doc.insert("target", self.target.into());

        if let Some(members) = self.members.as_ref() {
            let mut arr = Array::new();
            for member in members {
                arr.push(member);
            }
            doc.insert("members", value(arr));
        }

        if let Some(compiles) = self.compiles.as_ref() {
            let mut arr = Array::new();
            for compile in compiles {
                arr.push(Value::Integer(Formatted::new(*compile as i64)));
            }
            doc.insert("compiles", value(arr));
        }

        doc
    }
}

impl Display for RactToml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_toml().to_string().as_str())
    }
}

impl TryFrom<&DocumentMut> for RactToml {
    type Error = Error;

    fn try_from(value: &DocumentMut) -> Result<Self, Self::Error> {
        let target = value.get("target").map_or_else(
            || Err(ParseError::new("can not get target in toml", ParseType::Toml).into()),
            |v| {
                v.as_str().map_or_else(
                    || Err(ParseError::new("target must be a string", ParseType::Toml).into()),
                    |s| FrameworkType::from_str(s),
                )
            },
        )?;

        let members = if let Some(v) = value.get("members") {
            let member = v.as_array().map_or_else(
                || {
                    Err(Error::Parse(ParseError::new(
                        "members must be a array",
                        ParseType::Toml,
                    )))
                },
                |arr| {
                    let mut members = vec![];
                    for item in arr.iter() {
                        members.push(Member::try_from(item)?);
                    }
                    Ok(members)
                },
            )?;

            Some(member)
        } else {
            None
        };

        let compiles = if let Some(v) = value.get("compiles") {
            let compiles = v.as_array().map_or_else(
                || {
                    Err(Error::Parse(ParseError::new(
                        "compiles must be a array",
                        ParseType::Toml,
                    )))
                },
                |arr| {
                    let mut compiles = vec![];
                    for item in arr.iter() {
                        compiles.push(item.as_integer().map_or_else(
                            || Err(Error::from("compiles must be a integer")),
                            |i| Ok(i as usize),
                        )?);
                    }
                    Ok(compiles)
                },
            )?;

            Some(compiles)
        } else {
            None
        };

        Ok(Self {
            target,
            members,
            compiles,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Member {
    /// path of the source project which required to compile
    pub source: PathBuf,
    /// path of the project which after compiled
    pub target: PathBuf,
}

impl From<&Member> for Value {
    fn from(member: &Member) -> Self {
        let mut table = InlineTable::new();

        table.insert(
            "source",
            Value::String(Formatted::new(fs::path_to_str(member.source.as_path()))),
        );
        table.insert(
            "target",
            Value::String(Formatted::new(fs::path_to_str(member.target.as_path()))),
        );

        Value::InlineTable(table)
    }
}

impl From<(&ProjectInfo, usize)> for Member {
    fn from(value: (&ProjectInfo, usize)) -> Self {
        let (info, index) = value;
        Self {
            source: PathBuf::from(&info.name),
            target: PathBuf::from(format!("src_gen_{}", index)),
        }
    }
}

impl TryFrom<&Value> for Member {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| Error::from("Member must be a inline table".to_string()))?;

        let source = table.get("source").map_or_else(
            || Err(Error::from("can not get source in member".to_string())),
            |v| {
                v.as_str().map_or_else(
                    || Err(Error::from("source must be a string".to_string())),
                    |s| Ok(PathBuf::from(s)),
                )
            },
        )?;

        let target = table.get("target").map_or_else(
            || Err(Error::from("can not get target in member".to_string())),
            |v| {
                v.as_str().map_or_else(
                    || Err(Error::from("target must be a string".to_string())),
                    |s| Ok(PathBuf::from(s)),
                )
            },
        )?;

        Ok(Self { source, target })
    }
}

#[cfg(test)]
mod test_ract {
    use toml_edit::DocumentMut;

    use crate::core::entry::RactToml;

    #[test]
    fn makepad() {
        let input = r#"
        target = "makepad"
        "#;

        let toml = input.parse::<DocumentMut>().unwrap();
        let ract = RactToml::try_from(&toml).unwrap();
        println!("{}", ract);
    }

    #[test]
    fn gen_ui() {
        let input = r#"
        target = "gen_ui"
        members = [
            { source = "./hello", target = "./hello_makepad" },
            { source = "./world", target = "./world_makepad" },
        ]
        compiles = [0, 1]
        "#;

        let toml = input.parse::<DocumentMut>().unwrap();
        let ract = RactToml::try_from(&toml).unwrap();
        println!("{}", ract);
    }
}
