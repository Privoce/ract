use std::{env::current_dir, path::PathBuf, process::exit};

use gen_utils::{
    common::{fs, git_download_plugin_from_github, ToToml},
    error::Error,
};

use crate::core::{
    entry::{GenUIConf, Language, RactToml},
    log::{AddLogs, LogExt, TerminalLogger},
};

pub fn run(name: &str) {
    let lang = Language::from_conf();
    match download_and_update(name) {
        Ok(_) => {
            AddLogs::Complete(name.to_string()).terminal(&lang).success();
        }
        Err(e) => {
            AddLogs::DownloadFailed(e.to_string()).terminal(&lang).error();
        }
    }
}

fn download_and_update(name: &str) -> Result<(), Error> {
    let _ = download_plugins_from_github(name)?;
    AddLogs::DownloadSuccess(name.to_string())
        .terminal()
        .success();
    // write use in gen_ui.toml
    return update_plugin_in_toml(name);
}

/// ## update plugin in gen_ui.toml
/// if add gen_makepad_http, then write it in gen_ui.toml
/// ```toml
/// [plugins]
/// gen_makepad_http = ".plugins/gen_makepad_http"
/// ```
pub fn update_plugin_in_toml(plugin: &str) -> Result<(), Error> {
    let path = current_dir().unwrap();
    let ract_toml: RactToml = (&RactToml::read(path.join(".ract"))?).try_into()?;
    if let Some(compiles) = ract_toml.compiles() {
        let member = compiles[0];
        let source_path = path.join(member.source.as_path());
        // get gen_ui.toml
        let mut toml = GenUIConf::new(source_path.as_path())?;
        // write in gen_ui.toml
        toml.insert_plugin(
            plugin.to_string(),
            PathBuf::from(format!(".plugins/{}", plugin)),
        );
        // write back
        return toml.write(source_path.join("gen_ui.toml"));
    } else {
        return Err(Error::from(
            AddLogs::WriteInTomlFailed(plugin.to_string()).to_string(),
        ));
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

                return git_download_plugin_from_github(
                    plugin,
                    true,
                    download_path,
                    |line| TerminalLogger::new(&line).info(),
                    |line| TerminalLogger::new(&line).warning(),
                );
            }
        }
        crate::core::entry::FrameworkType::Makepad => {
            unimplemented!("Makepad does not support plugins yet")
        }
    }

    Ok(())
}
