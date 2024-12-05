/// Compiler target
mod target;
/// Compiler configuration
mod conf;
/// Exclude files or directories when compiling or watching
mod excludes;

pub use target::CompileTarget;
pub use conf::Conf as GenUIConf;