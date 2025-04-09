pub mod core;
use core::run_cli;

rust_i18n::i18n!("locales", fallback = ["en_US", "zh_CN"]);

fn main() {
    run_cli();
}
