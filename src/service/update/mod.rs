use std::path::Path;

use crate::{common::exe_path, entry::ChainEnvToml, log::LogItem};
use clap::Args;
use colored::Colorize;
use gen_utils::{common::shadow_cmd, error::Error};
use inquire::Confirm;

use super::uninstall;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(short, long, default_value = "false")]
    pub force: bool,
}

impl UpdateArgs {
    /// # 更新Ract工具链
    /// 查询crate.io上的最新版本，与本地版本进行比对，如果本地版本低于最新版本，提示用户更新
    pub fn run(&self) {
        match ask_for_update(self.force) {
            Ok(_) => {}
            Err(e) => {
                LogItem::error(format!("❌ Update failed! {}", e)).print();
            }
        }
    }
}

fn update() -> Result<(), Error> {
    // clear configs
    uninstall::uninstall_configs(exe_path()?)?;
    // run `cargo install ract --force`
    shadow_cmd(
        "cargo",
        ["install", "ract", "--force"],
        Option::<&Path>::None,
    )
}

/// 询问是否需要更新
fn ask_for_update(force: bool) -> Result<(), Error> {
    if force {
        return update();
    }
    let mut chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let (is_update, version) = chain_env_toml
        .check_force()
        .map_err(|e| Error::from(e.to_string()))?;
    if is_update {
        let (current, latest) = version.unwrap();
        println!(
            "❗️ Current version is {}\nthe latest version is {}",
            current.bright_yellow(),
            latest.bright_green()
        );
        let is_update = Confirm::new("Do you want to update?")
            .with_default(true)
            .prompt()
            .map_err(|e| Error::from(e.to_string()))?;

        if is_update {
            return update();
        }
    } else {
        LogItem::info("✅ No need to update!".to_string()).print();
    }

    Ok(())
}

pub fn check_auto_update() -> Result<(), Error> {
    let mut chain_env_toml: ChainEnvToml = ChainEnvToml::path()?.try_into()?;
    let (is_update, _version) = chain_env_toml
        .check()
        .map_err(|e| Error::from(e.to_string()))?;

    if is_update {
        // 需要进行更新
        if chain_env_toml.auto_update {
            // 自动更新
            return update();
        }
    }

    Ok(())
}
