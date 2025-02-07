use std::process::exit;

use gen_utils::error::Error;
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
    let exe_path = exe_path();
    // [配置文件] -----------------------------------------------------------------------------

    // [chain目录] ---------------------------------------------------------------------------

    // [Ract可执行文件] -----------------------------------------------------------------------

    Ok(())
}
