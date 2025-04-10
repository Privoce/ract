//! GenUI Watcher的目的是为了监听GenUI项目的变化，当GenUI项目发生变化时，GenUI Watcher会自动编译GenUI项目。
//! GenUI项目结构如下：
//! ```text
//! hello (workspace)
//! - src_gen (编译后的项目结果)
//! - hello (GenUI项目真实目录)
//!     - gen_ui.toml
//! - Cargo.toml (workspace 的Cargo.toml)
//! ```
//! 所以Watcher监听的是workspace下的`hello`目录
//! 在开启监视前会获取`hello`目录下的`gen_ui.toml`文件，然后根据`gen_ui.toml`文件中的`[watcher]`配置来进行监听。

use std::path::Path;

use crate::core::entry::compiler::excludes::Excludes;
#[cfg(target_os = "macos")]
use gen_utils::common::fs::FileState;
use gen_utils::error::Error;

/// ## init watcher
/// init watcher to watch file change event
/// - f: callback function, we can do something when file change
#[cfg(not(target_os = "macos"))]
pub fn init_watcher<P, F>(
    path: P,
    excludes: &Excludes,
    mut f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    F: FnMut(&Path, &notify::EventKind) -> Result<(), Error>,
{
    use std::{sync::mpsc::channel, time::Duration};

    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    use crate::core::log::compiler::{CompilerLogger, CompilerLogs};

    let (tx, rx) = channel();
    // [config for watcher] --------------------------------------------------------------------------------
    let config = Config::default();
    config.with_poll_interval(Duration::from_secs(10));
    // [watcher] -------------------------------------------------------------------------------------------
    let mut watcher = RecommendedWatcher::new(tx, config)?;
    // let mut fs_state = get_current_state(path)?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    CompilerLogs::WatcherInit(path.as_ref().to_path_buf())
        .compiler()
        .info();

    while let Ok(event) = rx.recv() {
        match event {
            Ok(event) => {
                // filter exclude
                if !excludes.contains(path.as_ref(), &event.paths[0]) {
                    if let Err(e) = f(&event.paths[0], &event.kind) {
                        CompilerLogger::new(&e.to_string()).error();
                    }
                }
            }
            Err(e) => {
                CompilerLogger::new(&e.to_string()).warn();
            }
        }
    }

    Ok(())
}

// let res = match event {
//     notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
//         self.do_compile(path)
//     }
//     notify::EventKind::Remove(_) => {
//         eprintln!("remove file: {:?}", path);
//         Ok(false)
//     },
//     _ => Ok(false),
// };

/// ## init watcher初始化编译器
/// 专属于Macos的编译事件需要进行防抖处理
#[cfg(target_os = "macos")]
pub fn init_watcher<P, F>(
    path: P,
    excludes: &Excludes,
    mut f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    F: FnMut(&Path, FileState) -> Result<(), Error>,
{
    use std::{sync::mpsc::channel, time::Duration};

    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    use crate::core::{
        entry::compiler::service::FileEventTracker,
        log::compiler::{CompilerLogger, CompilerLogs},
    };

    let (tx, rx) = channel();
    // [config for watcher] --------------------------------------------------------------------------------
    let config = Config::default();
    config.with_poll_interval(Duration::from_secs(10));
    // [watcher] -------------------------------------------------------------------------------------------
    let mut watcher = RecommendedWatcher::new(tx, config)?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    CompilerLogs::WatcherInit(path.as_ref().to_path_buf())
        .compiler()
        .info();

    // 初始化一个tracker
    let mut tracker = FileEventTracker::new();

    // 这里的事件处理需要进行防抖处理
    while let Ok(event) = rx.recv() {
        match event {
            Ok(event) => {
                let compiled_path = event.paths[0].to_path_buf();
                if !excludes.contains(path.as_ref(), compiled_path.as_path()) {
                    tracker.set_path(compiled_path.as_path());
                    tracker.insert(event.kind);
                    if let Some(state) = tracker.state() {
                        if let Err(e) = f(compiled_path.as_path(), state) {
                            CompilerLogger::new(&e.to_string()).error();
                        }else{
                            tracker.flesh();
                        }
                    }
                }
            }
            Err(e) => {
                CompilerLogger::new(&e.to_string()).warn();
            }
        }
    }

    Ok(())
}
