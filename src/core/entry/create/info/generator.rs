pub mod gen_ui;
pub mod makepad;

use super::ProjectInfoType;
use crate::core::entry::{FrameworkType, RactToml};
use gen_utils::error::Error;
use std::path::{Path, PathBuf};

pub struct Generator {
    pub path: PathBuf,
    pub info: ProjectInfoType,
    pub git: bool,
    pub framework: FrameworkType,
}

impl Generator {
    pub fn new<P>(path: P, info: ProjectInfoType, framework: FrameworkType) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
            info,
            git: false,
            framework,
        }
    }
    /// ## Generate target project
    pub fn generate(&self) -> Result<(), Error> {
        match self.framework {
            FrameworkType::GenUI => {
                // [ract.toml] -----------------------------------------------------------------
                let ract_toml = RactToml::gen_ui(self.info.members().unwrap());
            }
            FrameworkType::Makepad => {
                // [ract.toml] -----------------------------------------------------------------
                let ract_toml = RactToml::makepad();
                self.makepad(ract_toml)?;
            }
        };

        Ok(())
    }

    fn makepad(&self, ract_toml: RactToml) -> Result<(), Error> {
        match &self.info {
            ProjectInfoType::Workspace(workspace_info) => {
                makepad::create_workspace(self.path.as_path(), workspace_info, &ract_toml)
            }
            ProjectInfoType::Project(project_info) => {
                makepad::crate_project(self.path.as_path(), project_info)
            }
        }
    }
}
