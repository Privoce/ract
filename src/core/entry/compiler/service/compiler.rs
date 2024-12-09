use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use gen_utils::{
    common::ToToml,
    compiler::CompilerImpl,
    error::{Error, FsError},
};

use crate::core::entry::{GenUIConf, Member, Underlayer};

/// # GenUI Compiler
/// compiler will compile the file when the file is created or modified
///
/// but it will not compile the dir, only compile the file in the dir
///
/// dir will be generated after the file in the dir is compiled
pub struct Compiler {
    /// path of the compiled project and after compiled project
    pub source: Member,
    /// compiler target, default is makepad
    /// which depends on `gen_ui.toml` file
    pub target: Box<dyn CompilerImpl>,
}

impl Compiler {
    pub fn new<P>(path: P, member: &Member) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conf_path = path.as_ref().join(member.source.as_path());
        let target = GenUIConf::read(conf_path.as_path())?
            .get("target")
            .map(|target| {
                target.as_str().map_or_else(
                    || Err(Error::from("can not get target from .ract file")),
                    |target| Underlayer::from_str(target),
                )
            })
            .map_or_else(
                || {
                    Err(FsError::Read {
                        path: conf_path,
                        reason: ".ract file target can not find or parse error".to_string(),
                    }
                    .into())
                },
                |underlayer| underlayer,
            )?
            .compiler();

        Ok(Self {
            source: member.clone(),
            target,
        })
    }
}

impl CompilerImpl for Compiler {
    fn execute_auxiliaries(&mut self, executor: gen_utils::compiler::Executor) -> () {
        todo!()
    }

    fn exist_or_create(&self) -> () {
        todo!()
    }

    fn before_compile(&mut self) -> () {
        todo!()
    }

    fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) -> () {
        todo!()
    }

    fn insert(&mut self, node: Box<dyn std::any::Any>) -> () {
        todo!()
    }

    fn get(
        &self,
        key: &gen_utils::common::Source,
    ) -> Option<Box<dyn gen_utils::compiler::ModelNodeImpl>> {
        todo!()
    }
}
