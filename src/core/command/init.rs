use crate::core::{
    entry::{ChainEnvToml, Env},
    log::InitLogs,
};

pub fn check() {
    if Env::check() {
        return;
    } else {
        run();
    }
}

pub fn run() {
    InitLogs::Init.terminal().info();
    create_env_file();
    create_chain();
    InitLogs::Confirm.terminal().success();
}

fn create_env_file() {
    Env::default().write().expect("write env.toml failed");
    InitLogs::Env.terminal().success();
}

fn create_chain() {
    ChainEnvToml::default()
        .write()
        .expect("write env.toml failed");
    InitLogs::Chain.terminal().success();
}
