mod project;
mod workspace;

use gen_utils::error::Error;
use inquire::Select;
pub use project::ProjectInfo;
pub use workspace::WorkspaceInfo;

use crate::core::entry::{FrameworkType, Member};

pub enum ProjectPackageType {
    Workspace(WorkspaceInfo),
    Project(ProjectInfo),
}

impl ProjectPackageType {
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

        // [create project] -----------------------------------------------------------
        Self::create_project(project_type)
    }
    pub fn create_project(ty: &str) -> Result<Self, Error> {
        match ty {
            "workspace" => Ok(WorkspaceInfo::new().into()),
            "project" => Ok(ProjectInfo::new().into()),
            _ => Err(Error::from("Invalid project type")),
        }
    }
    pub fn members(&self) -> Option<Vec<Member>> {
        if let ProjectPackageType::Workspace(info) = self {
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

impl From<WorkspaceInfo> for ProjectPackageType {
    fn from(info: WorkspaceInfo) -> Self {
        ProjectPackageType::Workspace(info)
    }
}

impl From<ProjectInfo> for ProjectPackageType {
    fn from(info: ProjectInfo) -> Self {
        ProjectPackageType::Project(info)
    }
}
