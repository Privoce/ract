use std::fmt::Display;

use colored::Colorize;

#[derive(Debug)]
pub struct Tools {
    pub basic: BasicTools,
    pub underlayer: UnderlayerTools,
}

impl Display for Tools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}", self.basic, self.underlayer))
    }
}

impl Tools {
    pub fn options() -> Vec<&'static str> {
        vec!["rustc|cargo", "git", "makepad"]
    }
    pub fn is_ok(&self) -> bool {
        self.basic.is_ok() && self.underlayer.is_ok()
    }
}

#[derive(Debug, Default)]
pub struct BasicTools {
    pub rustc: bool,
    pub cargo: bool,
    pub git: bool,
}

impl BasicTools {
    pub fn is_ok(&self) -> bool {
        self.rustc && self.cargo && self.git
    }
}

impl From<(bool, bool, bool)> for BasicTools {
    fn from((rustc, cargo, git): (bool, bool, bool)) -> Self {
        Self { rustc, cargo, git }
    }
}

impl Display for BasicTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rustc = tag("rustc", self.rustc);
        let cargo = tag("cargo", self.cargo);
        let git = tag("git", self.git);

        f.write_fmt(format_args!(
            "\t🔻 Basic Tools\n\t\t🔹 {}\n\t\t🔹 {}\n\t\t🔹 {}",
            rustc, cargo, git
        ))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum UnderlayerTools {
    Makepad(MakepadTools),
    All(AllUnderlayer),
}

impl UnderlayerTools {
    pub fn is_ok(&self) -> bool {
        match self {
            UnderlayerTools::Makepad(makepad) => makepad.is_ok(),
            UnderlayerTools::All(all_underlayer) => all_underlayer.is_ok(),
        }
    }
    pub fn makepad_is_ok(&self) -> bool {
        match self {
            UnderlayerTools::Makepad(makepad) => makepad.makepad,
            UnderlayerTools::All(all_underlayer) => all_underlayer.makepad.makepad,
        }
    }
}

impl Display for UnderlayerTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnderlayerTools::Makepad(makepad_tools) => {
                f.write_fmt(format_args!("\t🔻 Makepad ToolChains:\n {}", makepad_tools))
            }
            UnderlayerTools::All(all_underlayer) => f.write_fmt(format_args!(
                "\t🔻 All Underlayer Tools:\n {}",
                all_underlayer
            )),
        }
    }
}

impl Default for UnderlayerTools {
    fn default() -> Self {
        UnderlayerTools::Makepad(MakepadTools::default())
    }
}

#[derive(Debug)]
pub struct AllUnderlayer {
    pub makepad: MakepadTools,
}

impl Display for AllUnderlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("🔸 Makepad ToolChains: {}", self.makepad))
    }
}

impl AllUnderlayer {
    pub fn is_ok(&self) -> bool {
        self.makepad.is_ok()
    }
}

#[derive(Debug, Default)]
pub struct MakepadTools {
    pub makepad: bool,
    pub gen_ui: bool,
}

impl MakepadTools {
    pub fn is_ok(&self) -> bool {
        self.makepad && self.gen_ui
    }
}

impl Display for MakepadTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let makepad = tag("makepad", self.makepad);
        let gen_ui = tag("gen-ui", self.gen_ui);

        f.write_fmt(format_args!("\t\t🔹 {}\n\t\t🔹 {}", makepad, gen_ui))
    }
}

impl From<(bool, bool)> for MakepadTools {
    fn from((makepad, gen_ui): (bool, bool)) -> Self {
        Self { makepad, gen_ui }
    }
}

fn tag(name: &str, status: bool) -> colored::ColoredString {
    return if status {
        format!("{}: {} ", name, "✅").green()
    } else {
        format!("{}: {} ", name, "❌").red()
    };
}
