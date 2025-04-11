mod constant;
mod fs;

pub use fs::*;
pub use constant::*;

use crate::log::error::Error;

pub type Result<T> = std::result::Result<T, Error>;