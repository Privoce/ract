use std::{error::Error, fmt::Display};

use super::terminal::TerminalLogger;

#[derive(Debug, Clone, Copy)]
pub enum WasmLogs {
    Welcome,
    Desc,
    Package,
    Start,
    Stop
}

impl Display for WasmLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmLogs::Welcome => f.write_str("🥳 Welcome to use ract wasm!"),
            WasmLogs::Desc => f.write_str(DESC),
            WasmLogs::Package => f.write_str("📦 wasm is being packaged"),
            WasmLogs::Start => f.write_str("🚀 wasm is being started"),
            WasmLogs::Stop => f.write_str("🛑 wasm is being stopped"),
        }
    }
}

impl WasmLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: std::borrow::Cow::Owned(self.to_string()),
        }
    }
}

const DESC: &str = r#"
🔸 Now only support makepad wasm
🔸 You can directly run in makepad project
🔸 If the project is in rust workspace, use -p to point target project
🔸 Or you can run `ract wasm` to build and start
"#;

impl Error for WasmLogs {}