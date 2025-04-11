use gen_utils::error::Error;
use inquire::{Confirm, Text};
use toml_edit::{value, Array, DocumentMut, Item, Table};

use crate::log::TerminalLogger;

use super::ProjectInfo;

/// WorkspaceInfo
/// help ract create a rust workspace project
#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    /// workspace name
    pub name: String,
    /// project info for workspace members
    pub members: Vec<ProjectInfo>,
}

impl WorkspaceInfo {
    pub fn new(is_gen_ui: bool) -> Result<WorkspaceInfo , Error>{
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
            TerminalLogger::new(
                format!("============ Project{} ======================", index).as_str(),
            )
            .warning();
            let project = ProjectInfo::new(is_gen_ui)?;
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

        Ok(workspace)
    }
    /// ## get workspace members
    /// It will return toml content (DocumentMut)
    pub fn workspace_members_toml(&self) -> DocumentMut {
        let mut toml = DocumentMut::new();
        let mut workspace = Table::new();
        let members = self.members.iter().fold(Array::new(), |mut arr, member| {
            arr.push(member.name.to_string());
            arr
        });
        workspace.insert("members", value(members));
        toml.insert("workspace", Item::Table(workspace));

        toml
    }
}
