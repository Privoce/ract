use std::path::PathBuf;

use gen_utils::common::Source;

#[allow(unused)]
pub trait CompilerSourceExt {
    fn from_path(&self) -> PathBuf;
    fn to_path(&self) -> PathBuf;
}

impl CompilerSourceExt for Source {
    fn from_path(&self) -> PathBuf {
        self.path.join(self.from.as_path())
    }

    fn to_path(&self) -> PathBuf {
        self.path.join(self.to.as_path())
    }
}
