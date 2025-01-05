use std::{env::current_dir, process::exit};

use gen_utils::{
    common::{fs, git_download_from_github, ToToml},
    error::Error,
};

use crate::core::{
    entry::RactToml,
    log::{AddLogs, TerminalLogger},
};

pub fn run(name: &str) {
    match download_plugins_from_github(name) {
        Ok(_) => {
            AddLogs::DownloadSuccess(name.to_string())
                .terminal()
                .success();
        }
        Err(e) => {
            AddLogs::DownloadFailed(e.to_string()).terminal().error();
            exit(2);
        }
    }
}

/// ## download plugins from github
/// use github api to download plugins from github
/// - repo: https://github.com/Privoce/genui_plugins
/// - dir: tokens
/// - branch: main
pub fn download_plugins_from_github(plugin: &str) -> Result<(), Error> {
    let path = current_dir().unwrap();
    let ract_toml: RactToml = (&RactToml::read(path.as_path().join(".ract"))?).try_into()?;

    match &ract_toml.target {
        crate::core::entry::FrameworkType::GenUI => {
            if let Some(compiles) = ract_toml.compiles() {
                let member = compiles[0];
                // 获取GenUI项目源码路径
                let source_path = path.join(&member.source);
                // 检查项目中是否存在.plugins目录，否则创建
                let download_path = source_path.join(".plugins");
                fs::exists_or_create_dir(download_path.as_path())?;
                // 从github仓库中下载指定的包，例如: ract add gen_makepad_http
                AddLogs::Downloading(plugin.to_string()).terminal().info();

                return git_download_from_github(
                    plugin,
                    true,
                    download_path,
                    |line| TerminalLogger::new(&line).info(),
                    |line| TerminalLogger::new(&line).warning(),
                );

                // let download_url = format!("tokens/{}/*", plugin);

                // download_from_github(
                //     download_url,
                //     &download_path,
                //     |line| TerminalLogger::new(&line).info(),
                //     |line| TerminalLogger::new(&line).warning(),
                // )?;
                // // 将下载好的包转移到.plugins目录下并删除.tmp目录
                // let from_path = download_path.join(".tmp").join("tokens");
                // fs::move_to(from_path, download_path.as_path())?;
                // fs::delete_dir(&download_path.join(".tmp"))?;
                // return Ok(());
            }
        }
        crate::core::entry::FrameworkType::Makepad => {
            unimplemented!("Makepad does not support plugins yet")
        }
    }

    Ok(())
}
