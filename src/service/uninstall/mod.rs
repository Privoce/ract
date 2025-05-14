use std::path::Path;
use gen_utils::{
    common::{cargo_install_list, fs, shadow_cmd},
    error::Error,
};
use crate::common::exe_path;

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
