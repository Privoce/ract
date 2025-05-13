use std::{error::Error, fmt::Display};

use rust_i18n::t;

use super::LogExt;

#[derive(Debug, Clone)]
pub enum WasmLogs {
    Desc,
    Package,
    Start,
    Stop,
    StopUnexpected(String),
    Port,
    PortError(String),
    Placeholder,
    NoRactConf,
}

impl Display for WasmLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.t(&crate::entry::Language::default()).as_ref())
    }
}

impl LogExt for WasmLogs {
    fn t(&self, lang: &crate::entry::Language) -> std::borrow::Cow<str> {
        let lang_str = lang.as_str();
        match self {
            WasmLogs::Desc => t!("wasm.desc", locale = lang_str),
            WasmLogs::Package => t!("wasm.package", locale = lang_str),
            WasmLogs::Start => t!("wasm.start", locale = lang_str),
            WasmLogs::Stop => t!("wasm.stop", locale = lang_str),
            WasmLogs::StopUnexpected(reason) => {
                t!("wasm.stop_unexpected", locale = lang_str, reason = reason)
            }
            WasmLogs::Port => t!("wasm.port", locale = lang_str),
            WasmLogs::PortError(reason) => {
                t!("wasm.port_err", locale = lang_str, reason = reason)
            }
            WasmLogs::Placeholder => t!("wasm.placeholder", locale = lang_str),
            WasmLogs::NoRactConf => t!("wasm.no_ract_conf", locale = lang_str),
        }
    }
}

impl Error for WasmLogs {}
