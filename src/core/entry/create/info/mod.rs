mod project;
mod workspace;

use gen_utils::error::Error;
use inquire::Select;
pub use project::ProjectInfo;
pub use workspace::WorkspaceInfo;

use crate::core::entry::ProjectType;

pub enum ProjectPackageType {
    Workspace(WorkspaceInfo),
    Project(ProjectInfo),
}

impl ProjectPackageType {
    pub fn new() -> Result<(), Error> {
        let framwork = Select::new(
            "Which framework template do you want to create?",
            ProjectType::options(),
        )
        .with_starting_cursor(0)
        .prompt()
        .unwrap();



        Ok(())

    }
}
