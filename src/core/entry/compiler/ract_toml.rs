use std::{fmt::Display, path::PathBuf, str::FromStr};

use gen_utils::common::{fs, ToToml};
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
