
mod project;
mod workspace;
mod generator;

pub use generator::Generator as ProjectGenerator;
use std::path::Path;
use gen_utils::error::Error;
use inquire::Select;
pub use project::ProjectInfo;
pub use workspace::WorkspaceInfo;

use crate::core::entry::{FrameworkType, Member};

#[derive(Debug, Clone)]
pub enum ProjectInfoType {
    Workspace(WorkspaceInfo),
    Project(ProjectInfo),
}

impl ProjectInfoType {
    /// create a new project
    pub fn create<P>(&self, path: P, framework: FrameworkType) -> ProjectGenerator
    where
        P: AsRef<Path>,
    {
        ProjectGenerator::new(path, self.clone(), framework)
    }
    pub fn new(framework: FrameworkType) -> Result<Self, Error> {
        // only makepad need to select project type
        let project_type = if let FrameworkType::Makepad = framework {
            // [select project type (workspace or project)] -------------------------------
            Select::new("Workspace or Bin Project?", Self::options())
                .with_starting_cursor(0)
                .prompt()
                .unwrap()
        } else {
            "workspace"
        };

        // [project info] -----------------------------------------------------------
        Self::project_info(project_type)
    }
    pub fn project_info(ty: &str) -> Result<Self, Error> {
        match ty {
            "workspace" => Ok(WorkspaceInfo::new().into()),
            "project" => Ok(ProjectInfo::new().into()),
            _ => Err(Error::from("Invalid project type")),
        }
    }
    pub fn members(&self) -> Option<Vec<Member>> {
        if let ProjectInfoType::Workspace(info) = self {
            return Some(
                info.members
                    .iter()
                    .enumerate()
                    .map(|(index, member)| (member, index).into())
                    .collect::<Vec<Member>>(),
            );
        }
        None
    }
    pub fn options() -> Vec<&'static str> {
        vec!["workspace", "project"]
    }
}

impl From<WorkspaceInfo> for ProjectInfoType {
    fn from(info: WorkspaceInfo) -> Self {
        ProjectInfoType::Workspace(info)
    }
}

impl From<ProjectInfo> for ProjectInfoType {
    fn from(info: ProjectInfo) -> Self {
        ProjectInfoType::Project(info)
    }
}
