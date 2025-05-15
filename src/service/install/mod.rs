mod git;
mod rustc;
mod underlayer;

use crate::{
    cli::command::install::InstallOptions,
    common::is_empty_dir,
    entry::{ChainEnvToml, Language, Tools},
};
use gen_utils::error::Error;
pub use git::*;
pub use rustc::*;

pub fn run(options: InstallOptions, lang: Language) -> Result<(), Error> {
    let chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let path = chain_env_toml.chain_path();
    // first you must install makepad (so check and ask user need to update?)
    let makepad_widgets_path = chain_env_toml.makepad_widgets_path();
    let makepad_is_ok = !is_empty_dir(makepad_widgets_path)?;
    // install depend on options
    for option in options {
        match option {
            Tools::Basic(basic_tools) => match basic_tools {
                crate::entry::BasicTools::Ructc => {
                    install_rustc(lang)?;
                }
                crate::entry::BasicTools::Git => {
                    install_git(lang)?;
                }
            },
            Tools::Underlayer(underlayer_tools) => match underlayer_tools {
                crate::entry::UnderlayerTools::Makepad(makepad_tools) => {
                    underlayer::install(path.as_path(), makepad_is_ok, makepad_tools, lang)?;
                }
            },
        }
    }
    Ok(())
}
