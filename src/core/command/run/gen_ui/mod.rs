use std::path::{Path, PathBuf};

use gen_utils::{compiler, error::Error};

pub fn run<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let path = target_path(path);
    // [generate compiler service] -----------------------------------------------------------------------
    let compilerpiler = Compiler::new()

    Ok(())
}

fn target_path<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_path_buf();
    let last = path.iter().last().unwrap();
    path.join(last)
}
