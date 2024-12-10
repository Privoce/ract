use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use gen_utils::{
    common::ToToml,
    compiler::CompilerImpl,
    error::{Error, FsError},
};

use crate::core::{
    constant::LOGO,
    entry::{GenUIConf, Member, Underlayer},
    log::{compiler::{CompilerLogger, CompilerLogs}, TerminalLogger},
};

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
    /// gen_ui.toml file conf
    pub conf: GenUIConf,
}

impl Compiler {
    pub fn new<P>(path: P, member: &Member) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let source_path = path.as_ref().join(member.source.as_path());
        // [conf] ----------------------------------------------------------------------------------------
        let conf: GenUIConf = (&GenUIConf::read(source_path.join("gen_ui.toml"))?).try_into()?;
        // [target] --------------------------------------------------------------------------------------
        let target = conf.compiler.target.compiler();

        Ok(Self {
            source: member.clone(),
            target,
            conf,
        })
    }
    /// run compiler
    /// - init and execute watcher
    pub fn run(&mut self) {
        self.before_compile();
        // info(APP_RUNNING);
        // let rt = Runtime::new().unwrap();
        // let origin_path = self.origin_path.clone();
        // let excludes = self.exclude.clone();
        // rt.block_on(async {
        //     if let Err(e) =
        //         init_watcher(origin_path.as_path(), &excludes, |path, e_kind, f_kind| {
        //             match e_kind {
        //                 notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
        //                     // create or modify
        //                     self.compile_one(path);
        //                 }
        //                 notify::EventKind::Remove(_) => {
        //                     // remove from cache and compiled project, after test we know, only remove need f_kind to know the file is dir or file
        //                     self.remove_compiled(path, f_kind);
        //                 }
        //                 _ => (),
        //             }
        //             // do other auxiliary work
        //             let _ = self.execute_auxiliaries(Executor {
        //                 success: Box::new(|msg| {
        //                     info(msg);
        //                 }),
        //                 fail: Box::new(|e| error(e.to_string().as_str())),
        //                 ignore: Box::new(|| {
        //                     ();
        //                 }),
        //             });
        //         })
        //         .await
        //     {
        //         // log error and stop the service
        //         error(e.to_string().as_str());
        //         return;
        //     }
        // });
        // exit(-1);
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
        // [display LOGO] ------------------------------------------------------------------------------------------------
        if self.conf.compiler.logo {
            CompilerLogs::Logo.terminal().logo();
        }
        // [init logger] ------------------------------------------------------------------------------------------------
        let log_level = self.conf.compiler.log_level;
        let _ = crate::core::log::compiler::init(log_level);
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
