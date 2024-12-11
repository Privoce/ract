use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
};

use gen_utils::{
    common::{
        fs::{self, copy_file, GenUIFs}, read_to_doc, Source, ToToml
    },
    compiler::CompilerImpl,
    error::Error,
};
use toml_edit::{value, DocumentMut};

use crate::core::{
    entry::{GenUIConf, Member},
    log::compiler::CompilerLogs,
};

use super::{init_watcher, Cache, CompilerSourceExt};

/// # GenUI Compiler
/// compiler will compile the file when the file is created or modified
///
/// but it will not compile the dir, only compile the file in the dir
///
/// dir will be generated after the file in the dir is compiled
pub struct Compiler {
    pub source: Source,
    /// compiler target, default is makepad
    /// which depends on `gen_ui.toml` file
    pub target: Box<dyn CompilerImpl>,
    /// gen_ui.toml file conf
    pub conf: GenUIConf,
    /// cache of the compiled project
    pub cache: Cache,
}

impl Compiler {
    pub fn new<P>(path: P, member: &Member) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let source_path = path.as_ref().join(member.source.as_path());
        // [source] --------------------------------------------------------------------------------------
        let source = member.to_source(path.as_ref());
        // [conf] ----------------------------------------------------------------------------------------
        let conf: GenUIConf = (&GenUIConf::read(source_path.join("gen_ui.toml"))?).try_into()?;
        // [target] --------------------------------------------------------------------------------------
        let target = conf
            .compiler
            .target
            .compiler(&source, &conf.underlayer.target)?;
        // [cache] ---------------------------------------------------------------------------------------
        let cache = Cache::new(&source_path)?;

        Ok(Self {
            source,
            target,
            conf,
            cache,
        })
    }
    /// run compiler
    /// - init and execute watcher
    pub fn run(&mut self) {
        self.before_compile();
        // [compiler source path] -------------------------------------------------------------------------
        let source = self.source.from_path();
        // [init watcher] ---------------------------------------------------------------------------------
        let excludes = self.conf.compiler.excludes.clone();
        let _ = init_watcher(source, &excludes, |path, event| match event {
            notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                let _ = self.compile_one(path);
            }
            notify::EventKind::Remove(_) => {}
            _ => (),
        });
        exit(1);
    }

    /// compile single gen / other type file
    fn compile_one<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut compiled = false;
        let source_path = self.source.from_path();
        //  let target_path = self.origin_path.as_path().to_path_buf();
        match (path.as_ref().is_file(), path.as_ref().is_gen_file()) {
            (false, true) | (false, false) => {
                // if is dir, do nothing , use lazy compile(only dir has file, file will be compiled, dir generate after file compiled)
                return Ok(());
            }
            (true, true) => {
                self.cache
                    .exists_or_insert(path.as_ref())
                    .unwrap()
                    .modify_then(|| {
                        // let model =
                        //     Model::new(&path.as_ref().to_path_buf(), &target_path, false).unwrap();
                        // let source = model.special.clone();
                        // let _ = self.insert(Box::new(model));
                        // let _ = self.get(&source).unwrap().compile();
                        compiled = true;
                    });
            }
            (true, false) => {
                // not gen file, directly copy to the compiled project
                let compiled_path = path.as_ref().to_compiled(
                    self.source.path.as_path(),
                    self.source.from.as_path(),
                    self.source.to.as_path(),
                )?;

                let _ = self
                    .cache
                    .exists_or_insert(path.as_ref())
                    .unwrap()
                    .modify_then(|| {
                        let _ = copy_file(path.as_ref(), compiled_path);
                        compiled = true;
                    });
            }
        }

        if compiled {
            let _ = self.cache.write(source_path.as_path());
            CompilerLogs::Compiled(path.as_ref().to_path_buf())
                .compiler()
                .info();
        }

        Ok(())
    }

    // fn loop_compile(compiler: &mut Compiler, visited: &mut HashSet<PathBuf>) {
    //     // Convert to absolute path
    //     // let target_path = target.as_ref().canonicalize().unwrap();
    //     let target_path = compiler.origin_path.as_path().to_path_buf();
    //     if !visited.insert(target_path.clone()) {
    //         return;
    //     }

    //     for item in WalkDir::new(target_path.as_path())
    //         .into_iter()
    //         .filter_map(|d| d.ok())
    //     {
    //         let source_path = item.path();
    //         // check if the file or folder is in the exclude list, if true, skip it
    //         if compiler
    //             .exclude
    //             .iter()
    //             .any(|uncompiled_item| is_eq_path_exclude(source_path, uncompiled_item.as_path()))
    //         {
    //             continue;
    //         }

    //         match (
    //             source_path.is_file(),
    //             source_path.to_str().unwrap().ends_with(".gen"),
    //         ) {
    //             (false, true) | (false, false) => {
    //                 // is dir should loop compile again
    //                 Compiler::loop_compile(compiler, visited);
    //             }
    //             (true, true) => {
    //                 compiler
    //                     .cache
    //                     .exists_or_insert(source_path)
    //                     .unwrap()
    //                     .then(|_| {
    //                         let model = Model::new(&source_path.to_path_buf(), &target_path, false)
    //                             .unwrap();
    //                         let _ = compiler.insert(Box::new(model));
    //                     });

    //                 // let model =
    //                 //     Model::new(&source_path.to_path_buf(), &target_path, false).unwrap();
    //                 // let _ = compiler.insert(Box::new(model));
    //             }
    //             (true, false) => {
    //                 // is file but not gen file, directly copy to the compiled project
    //                 // get the compiled path
    //                 let compiled_path = Source::origin_file_without_gen(source_path, &target_path);
    //                 // check and insert into cache
    //                 let _ = compiler
    //                     .cache
    //                     .exists_or_insert(source_path)
    //                     .unwrap()
    //                     .modify_then(|| {
    //                         let _ = copy_file(source_path, compiled_path);
    //                     });
    //             }
    //         }
    //     }
    // }
}

impl CompilerImpl for Compiler {
    fn execute_auxiliaries(&mut self, executor: gen_utils::compiler::Executor) -> () {
        todo!()
    }

    /// ## check if the generate rust project exists, if not create one
    ///
    /// ### details
    /// - check if the project exists which named "src_gen"
    ///     - true: return true
    ///     - false: create a new rust project named "src_gen"
    /// - and need to check whether the super project is a rust workspace project
    ///     - if not, panic and tell the user to create a workspace project
    ///     - if true, check and add the "src_gen" project to the workspace member list
    /// ### test
    /// - no src_gen: 👌
    /// - no src_gen and no workspace: 👌
    fn exist_or_create(&self) -> Result<(), Error> {
        // check the super project is a workspace project or not
        let target_project = self.source.to.to_str().unwrap().to_string();

        let workspace_toml_path = self.source.path.join("Cargo.toml");

        if !workspace_toml_path.exists() {
            return Err(Error::from("Cargo.toml not found in the super project, you should create a workspace project first"));
        } else {
            // read the super project's Cargo.toml file and check the workspace member list
            let mut workspace_toml = read_to_doc(workspace_toml_path.as_path())?;

            let member_list = workspace_toml
                .get_mut("workspace")
                .expect("workspace not found in Cargo.toml")
                .get_mut("members")
                .expect("members not found in Cargo.toml")
                .as_array_mut()
                .expect("members is not an array");

            // check member list contains the src_gen project or not
            if member_list
                .iter()
                .find(|item| item.as_str().unwrap() == &target_project)
                .is_none()
            {
                // add the src_gen project to the workspace member list
                member_list.push(&target_project);
            }

            // check workspace resolver exists or not, if not, add workspace.resolver = "2"
            if workspace_toml
                .get("workspace")
                .unwrap()
                .get("resolver")
                .is_none()
            {
                workspace_toml["workspace"]["resolver"] = value("2");
            }

            // write back
            fs::write(workspace_toml_path.as_path(), &workspace_toml.to_string())
                .expect("failed to write super project's Cargo.toml");
        }

        // check the target project exists or not
        if !self
            .source
            .path
            .as_path()
            .join(target_project.as_str())
            .exists()
        {
            // use std::process::Command to create a new rust project
            let status = Command::new("cargo")
                .args(["new", "--bin", target_project.as_str()])
                .current_dir(self.source.path.as_path())
                .status()
                .expect("failed to create target project");

            if !status.success() {
                panic!("failed to create target project");
            }
        }
        // now call target exist_or_create
        self.target.exist_or_create()
    }

    fn before_compile(&mut self) -> () {
        // [display LOGO] ------------------------------------------------------------------------------------------------
        if self.conf.compiler.logo {
            CompilerLogs::Logo.terminal().logo();
        }
        // [init logger] -------------------------------------------------------------------------------------------------
        let log_level = self.conf.compiler.log_level;
        let _ = crate::core::log::compiler::init(log_level);
        // [check compiler target] ---------------------------------------------------------------------------------------
        self.exist_or_create();
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
