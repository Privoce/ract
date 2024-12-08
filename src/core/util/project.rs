use std::path::Path;

use gen_utils::error::Error;

pub fn create_workspace<P>(path: P)->Result<(), Error> where P: AsRef<Path> {
    Ok(())
}