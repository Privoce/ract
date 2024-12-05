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