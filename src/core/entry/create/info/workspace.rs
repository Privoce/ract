use std::path::PathBuf;

use crate::core::entry::ProjectType;

use super::ProjectInfo;

/// WorkspaceInfo
/// help ract create a rust workspace project
pub struct WorkspaceInfo{
    /// workspace name
    pub name: String,
    /// workspace path
    pub path: PathBuf,
    /// target framework type for project
    pub ty: ProjectType,
    /// project info for workspace members
    pub info: Vec<ProjectInfo>
}


impl WorkspaceInfo {

}
