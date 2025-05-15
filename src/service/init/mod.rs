use gen_utils::error::Error;

use crate::{
    entry::{ChainEnvToml, Env},
    log::{InitLogs, LogExt},
};

pub fn run() -> Result<(), Error> {
    let lang = crate::entry::Language::En;
    InitLogs::Init.info(lang).print();
    create_env_file()?;
    InitLogs::EnvSuccess.success(lang).print();
    create_chain()?;
    InitLogs::ChainSuccess.success(lang).print();
    InitLogs::Complete.success(lang).print();
    Ok(())
}

pub fn create_env_file() -> Result<(), Error> {
    Env::default().write()
}

pub fn create_chain() -> Result<(), Error> {
    ChainEnvToml::default().write()
}
