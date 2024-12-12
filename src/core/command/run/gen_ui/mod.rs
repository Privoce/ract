use std::path::Path;

use gen_utils::{compiler::CompilerImpl, error::Error};

use crate::core::entry::{Compiler, RactToml};

pub fn run<P>(path: P, ract_toml: &RactToml) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    if let Some(compiles) = ract_toml.compiles() {
        // TODO!(multi thread compiler) now use single compiler
        let member = compiles[0];
        // [generate compiler service] -----------------------------------------------------------------------
        let mut compiler = Compiler::new(path.as_ref(), member)?;

        compiler.run();

        Ok(())
    } else {
        Err("can not get compile members from .ract".to_string().into())
    }
}
