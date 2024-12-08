use std::path::Path;

use gen_utils::error::Error;

use super::ProjectInfoType;

pub fn create<P>(path: P, info: &ProjectInfoType) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    Ok(())
}