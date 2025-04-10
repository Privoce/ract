use std::path::PathBuf;

use clap::Args;

/// ## Create a new project at the current directory
/// 
/// Create a new project
/// This command will create a new project at the specified path
/// 
/// ```shell
/// ract create
/// ```
#[derive(Args, Debug)]
pub struct CreateArgs {
    // #[arg(short, long, default_value = "makepad")]
    // pub target: Underlayer,
    /// Path to create the project
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
}
