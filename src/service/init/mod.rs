use gen_utils::error::Error;

use crate::{
    entry::{ChainEnvToml, Env},
    log::{InitLogs, LogExt, TerminalLogger},
};

use super::update::check_auto_update;

pub fn check() {
    // check env.toml
    if Env::check() {
        // check update
        if let Err(e) = check_auto_update() {
            TerminalLogger::new(e.to_string().as_str()).error();
        }
        return;
    } else {
        run();
    }
}

pub fn run() -> Result<(), Error> {
    let lang = crate::entry::Language::En;
    InitLogs::Init.terminal(&lang).info();
    create_env_file()?;
    InitLogs::EnvSuccess.terminal(&lang).success();
    create_chain()?;
    InitLogs::ChainSuccess.terminal(&lang).success();
    InitLogs::Complete.terminal(&lang).success();
    Ok(())
}

pub fn create_env_file() -> Result<(), Error> {
    Env::default().write()
}

pub fn create_chain() -> Result<(), Error> {
    ChainEnvToml::default().write()
}
