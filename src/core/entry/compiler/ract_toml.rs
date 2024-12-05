use std::path::PathBuf;

/// # RactToml
/// each project has a .ract file to point the project kind and help ract to compile the project
/// ## Example
/// ```toml
/// target = "gen_ui"
/// members = [
///    { source = "./hello", target = "./hello_makepad" },
/// ]
/// compiles = [0]
/// ```
pub struct RactToml{
    /// target of the project
    pub target: Target,
    /// members of the project
    pub members: Option<Vec<Member>>,
    /// projects to compile, if not set, compile the first project in the members
    /// - if compiles length is 0, not compile any project
    /// - if compiles length is 1, compile the project in the members by index
    /// - if compiles length is more than 1, use multiple threads to compile the projects
    pub compiles: Option<Vec<usize>>,
}


#[derive(Debug, Clone, Copy, Default)]
pub enum Target{
    #[default]
    GenUI,
    Makepad
}

#[derive(Debug, Clone)]
pub struct Member{
    /// path of the source project which required to compile
    pub source: PathBuf,
    /// path of the project which after compiled
    pub target: PathBuf,
}