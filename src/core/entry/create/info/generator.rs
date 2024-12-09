pub mod gen_ui;
pub mod makepad;

use super::ProjectInfoType;
use crate::core::{
    constant::DEFAULT_GITIGNORE,
    entry::{FrameworkType, RactToml},
    log::CreateLogs,
};
use gen_utils::{common::fs, error::Error};
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
        // [ract.toml] -----------------------------------------------------------------
        let ract_toml = match self.framework {
            FrameworkType::GenUI => RactToml::gen_ui(self.info.members().unwrap()),
            FrameworkType::Makepad => RactToml::makepad(),
        };

        let path = self.makepad(ract_toml)?;

        if self.git {
            // [init git repository] ------------------------------------------------------
            let _ = self.git_init(path.as_path())?;
        }

        Ok(())
    }

    fn makepad(&self, ract_toml: RactToml) -> Result<PathBuf, Error> {
        match &self.info {
            ProjectInfoType::Workspace(workspace_info) => {
                makepad::create_workspace(self.path.as_path(), workspace_info, &ract_toml)
            }
            ProjectInfoType::Project(project_info) => {
                makepad::create_project(self.path.as_path(), project_info)
            }
        }
    }

    pub fn git_init<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        // init git repository
        std::process::Command::new("git")
            .arg("init")
            .current_dir(path.as_ref())
            .output()
            .map_or_else(
                |e| Err(Error::from(e.to_string())),
                |out| {
                    if out.status.success() {
                        // write .gitignore
                        let _ = fs::write(
                            path.as_ref().join(".gitignore").as_path(),
                            DEFAULT_GITIGNORE,
                        );
                        CreateLogs::Git.terminal().success();
                        Ok(())
                    } else {
                        Err(CreateLogs::GitErr.to_string().into())
                    }
                },
            )
    }
}
