use std::path::PathBuf;

use inquire::{Confirm, Text};

use crate::core::{entry::{FrameworkType, RactToml}, log::TerminalLogger};

use super::ProjectInfo;

/// WorkspaceInfo
/// help ract create a rust workspace project
#[derive(Debug)]
pub struct WorkspaceInfo {
    /// workspace name
    pub name: String,
    /// project info for workspace members
    pub members: Vec<ProjectInfo>,
}

impl WorkspaceInfo {
    pub fn new() -> WorkspaceInfo {
        // [workspace name] ---------------------------------------------------------
        let name = Text::new("Input the name of the workspace:")
            .prompt()
            .expect("Failed to get workspace name");

        let mut workspace = WorkspaceInfo {
            name,
            members: Vec::new(),
        };
        // [members] ----------------------------------------------------------------
        let mut index = 1;
        loop {
            TerminalLogger::new(format!("============ Project{} ======================", index).as_str()).warning();
            let project = ProjectInfo::new();
            workspace.members.push(project);
            index += 1;
            let continue_or = Confirm::new("Do you want to add another project?")
                .with_default(false)
                .prompt()
                .expect("Failed to get continue or not");

            if !continue_or {
                break;
            }
        }

        workspace
    }
}
