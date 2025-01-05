use std::{
    env::current_dir,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

use gen_utils::{
    common::{fs, stream_terminal, ToToml},
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
                let download_url = format!("tokens/{}/*", plugin);

                AddLogs::Downloading(plugin.to_string()).terminal().info();
                download_from_github(download_url, &download_path)?;
                // 将下载好的包转移到.plugins目录下并删除.tmp目录
                let from_path = download_path.join(".tmp").join("tokens");
                fs::move_to(from_path, download_path.as_path())?;
                fs::delete_dir(&download_path.join(".tmp"))?;
                return Ok(());
            }
        }
        crate::core::entry::FrameworkType::Makepad => {
            unimplemented!("Makepad does not support plugins yet")
        }
    }

    Ok(())
}

/// use git to download from github
pub fn download_from_github(url: String, path: &PathBuf) -> Result<(), Error> {
    // [init a tmp git repo for downloading] -----------------------------------------------------------------------
    let tmp_download_path = path.join(".tmp");
    fs::delete_dir(&tmp_download_path)?;
    fs::create_dir(&tmp_download_path)?;
    // - [git init .tmp] -------------------------------------------------------------------------------------------
    run_shadow_cmd("git", &["init"], Some(&tmp_download_path))?;
    // - [add remote] ----------------------------------------------------------------------------------------------
    run_shadow_cmd(
        "git",
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/Privoce/genui_plugins.git",
        ],
        Some(&tmp_download_path),
    )?;
    // - [config core.sparseCheckout true] --------------------------------------------------------------------------
    run_shadow_cmd(
        "git",
        &["config", "core.sparseCheckout", "true"],
        Some(&tmp_download_path),
    )?;
    // - [echo "dir" >> .git/info/sparse-checkout] ------------------------------------------------------------------
    let to = tmp_download_path
        .join(".git")
        .join("info")
        .join("sparse-checkout");
    fs::write(to.as_path(), &url)?;
    // [pull down] --------------------------------------------------------------------------------------------------
    let mut child = Command::new("git")
        .args(&["pull", "origin", "main"])
        .current_dir(&tmp_download_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    stream_terminal(
        &mut child,
        |line| TerminalLogger::new(&line).info(),
        |line| TerminalLogger::new(&line).warning(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::from("please check you network connection"))
            }
        },
    )
}

fn run_shadow_cmd<I, S, P>(name: &str, args: I, current_dir: Option<P>) -> Result<(), Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let mut cmd = Command::new(name);

    cmd.args(args);

    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }

    cmd.status().map_or_else(
        |e| Err(Error::from(e.to_string())),
        |status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::from("init git repo failed"))
            }
        },
    )
}
