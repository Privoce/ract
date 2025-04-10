// pub mod core;
pub mod cli;
pub mod app;
pub mod service;
pub mod entry;
pub mod log;
pub mod common;
// use core::run_cli;

rust_i18n::i18n!("locales", fallback = ["en_US", "zh_CN"]);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run_cli();
    app::app()
}
