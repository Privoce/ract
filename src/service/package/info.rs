use std::{collections::HashMap, path::{Path, PathBuf}};
use gen_utils::common::fs;
use crate::entry::{FrameworkType, PackageConf, Resource};

#[allow(unused)]
pub struct PackageInfo {
    /// target project path
    pub path: PathBuf,
    pub conf: PackageConf,
    pub resources: Vec<Resource>,
    pub framework: Option<FrameworkType>,
}

impl PackageInfo {
    pub fn new<P>(
        path: P,
        conf: PackageConf,
        framework: Option<FrameworkType>,
        resources: Vec<Resource>,
    ) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
            conf,
            framework,
            resources,
        }
    }

    /// ## zip ract resources and package conf resources
    /// two resources:
    /// 1. ract: {src: PathBuf, target: String}
    /// 2. package: {src: PathBuf, target: String}
    /// ract.target = package.src -> insert(ract.src, package)
    pub fn zip_resources(&self) -> Option<HashMap<String, (PathBuf, PathBuf)>> {
        self.conf.resources.as_ref().map(|pkg_resources| {
            self.resources
                .iter()
                .fold(HashMap::new(), |mut acc, ract_resource| {
                    if let Resource::Obj { src, target } = ract_resource {
                        let ract_ident = fs::path_to_str(src);
                        let ract_target = PathBuf::from(target);
                        for pkg_resource in pkg_resources {
                            if let Resource::Obj { src, target } = pkg_resource {
                                if ract_target.eq(src) {
                                    acc.insert(
                                        ract_ident.to_string(),
                                        (src.to_path_buf(), PathBuf::from(target)),
                                    );
                                }
                            }
                        }
                    }
                    acc
                })
        })
    }
}