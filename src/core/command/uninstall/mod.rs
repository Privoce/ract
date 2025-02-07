use std::{path::Path, process::exit};

use gen_utils::{
    common::{cargo_install_list, fs, shadow_cmd},
    error::Error,
};
use inquire::Confirm;

use crate::core::{log::TerminalLogger, util::exe_path};

/// 卸载Ract
/// 由于Ract是一个工具链，所以卸载Ract就是删除Ract的所有文件，包括:
/// 1. Ract的环境配置文件(.env)
/// 2. .env同级的chain目录(chain是链依赖目录，存放链依赖的环墫配置文件和相关的依赖包)
/// 3. Ract的可执行文件(name = ract)
pub fn run() -> () {
    // [提示用户是否确定卸载] --------------------------------------------------------------------
    let is_uninstall = Confirm::new("Are you sure to uninstall Ract?")
        .with_default(false)
        .prompt()
        .expect("confirm failed");
    // [卸载] ---------------------------------------------------------------------------------
    if is_uninstall {
        match uninstall_all() {
            Ok(_) => {
                TerminalLogger::new("Uninstall Ract success!").success();
            }
            Err(e) => {
                TerminalLogger::new(&e.to_string()).error();
                exit(2);
            }
        }
    }
}

pub fn uninstall_all() -> Result<(), Error> {
    let exe_path = exe_path()?;
    uninstall_configs(exe_path.as_path())?;
    // [Ract可执行文件] -----------------------------------------------------------------------
    // - [用户可能是用cargo安装的，所以需要用cargo检查是否有ract] ----------------------------------
    let cargo_bins = cargo_install_list()?;
    if cargo_bins.contains_key("ract") {
        // 执行cargo uninstall ract
        let _ = shadow_cmd("cargo", ["uninstall", "ract"], Some(exe_path.as_path()))?;
    } else {
        let ract_path = exe_path.join("ract");
        fs::delete(&ract_path)?;
    }
    Ok(())
}

pub fn uninstall_configs<P>(exe_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // [配置文件] -----------------------------------------------------------------------------
    fs::delete(exe_path.as_ref().join(".env"))?;
    // [chain目录] ---------------------------------------------------------------------------
    fs::delete_dir(exe_path.as_ref().join("chain"))?;
    Ok(())
}
