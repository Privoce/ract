use std::fmt::Display;

pub enum Tools {
    Basic(BasicTools),
    Underlayer(UnderlayerTools),
}

impl Tools {
    pub fn options() -> Vec<&'static str> {
        vec![
            "rustc|cargo",
            "git",
            "makepad",
            "gen_ui",
            "android_builder",
            "ios_builder",
            "wasm_builder",
            "makepad_studio",
        ]
    }
}

impl From<&str> for Tools {
    fn from(value: &str) -> Self {
        match value {
            "rustc|cargo" => Tools::Basic(BasicTools::Ructc),
            "git" => Tools::Basic(BasicTools::Git),
            "makepad" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::Makepad)),
            "gen_ui" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::GenUi)),
            "android_builder" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::Android)),
            "ios_builder" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::Ios)),
            "wasm_builder" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::Wasm)),
            "makepad_studio" => Tools::Underlayer(UnderlayerTools::Makepad(MakepadTools::Studio)),
            _ => unimplemented!("{} is not supported", value),
        }
    }
}

pub struct ToolState {
    pub basic: BasicState,
    pub underlayer: MakepadState,
}

impl Display for ToolState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "🔸 Basic Tools:\n\t\t{}\n\t\t{}\n\t\t{}\n",
            self.basic.rustc, self.basic.cargo, self.basic.git
        ))?;
        f.write_fmt(format_args!(
            "🔸 Makepad ToolChains:\n\t\t{}\n\t\t{}\n",
            self.underlayer.makepad, self.underlayer.gen_ui
        ))
    }
}

impl ToolState {
    pub fn is_ok(&self) -> bool {
        self.basic.is_ok() && self.underlayer.is_ok()
    }
}

pub struct BasicState {
    pub rustc: bool,
    pub cargo: bool,
    pub git: bool,
}

impl BasicState {
    pub fn new(rustc: bool, cargo: bool, git: bool) -> Self {
        Self { rustc, cargo, git }
    }
    pub fn is_ok(&self) -> bool {
        self.rustc && self.cargo && self.git
    }
}

impl From<(bool, bool, bool)> for BasicState {
    fn from((rustc, cargo, git): (bool, bool, bool)) -> Self {
        Self { rustc, cargo, git }
    }
}

// use std::fmt::Display;

// use colored::Colorize;

// #[derive(Debug)]
// pub struct Tools {
//     pub basic: BasicTools,
//     pub underlayer: UnderlayerTools,
// }

// impl Display for Tools {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}\n{}", self.basic, self.underlayer))
//     }
// }

// impl Tools {
//     pub fn options() -> Vec<&'static str> {
//         vec!["rustc|cargo", "git", "makepad"]
//     }
//     pub fn is_ok(&self) -> bool {
//         self.basic.is_ok() && self.underlayer.is_ok()
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub enum BasicTools {
    #[default]
    Ructc,
    Git,
}

// impl BasicTools {
//     pub fn is_ok(&self) -> bool {
//         self.rustc && self.cargo && self.git
//     }
// }

// impl From<(bool, bool, bool)> for BasicTools {
//     fn from((rustc, cargo, git): (bool, bool, bool)) -> Self {
//         Self { rustc, cargo, git }
//     }
// }

// impl Display for BasicTools {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let rustc = tag("rustc", self.rustc);
//         let cargo = tag("cargo", self.cargo);
//         let git = tag("git", self.git);

//         f.write_fmt(format_args!(
//             "\t🔻 Basic Tools\n\t\t🔹 {}\n\t\t🔹 {}\n\t\t🔹 {}",
//             rustc, cargo, git
//         ))
//     }
// }

// #[allow(dead_code)]
#[derive(Debug)]
pub enum UnderlayerTools {
    Makepad(MakepadTools),
}

// impl UnderlayerTools {
//     pub fn is_ok(&self) -> bool {
//         match self {
//             UnderlayerTools::Makepad(makepad) => makepad.is_ok(),
//             UnderlayerTools::All(all_underlayer) => all_underlayer.is_ok(),
//         }
//     }
//     pub fn makepad_is_ok(&self) -> bool {
//         match self {
//             UnderlayerTools::Makepad(makepad) => makepad.makepad,
//             UnderlayerTools::All(all_underlayer) => all_underlayer.makepad.makepad,
//         }
//     }
// }

// impl Display for UnderlayerTools {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UnderlayerTools::Makepad(makepad_tools) => {
//                 f.write_fmt(format_args!("\t🔻 Makepad ToolChains:\n {}", makepad_tools))
//             }
//             UnderlayerTools::All(all_underlayer) => f.write_fmt(format_args!(
//                 "\t🔻 All Underlayer Tools:\n {}",
//                 all_underlayer
//             )),
//         }
//     }
// }

// impl Default for UnderlayerTools {
//     fn default() -> Self {
//         UnderlayerTools::Makepad(MakepadTools::default())
//     }
// }

// #[derive(Debug)]
// pub struct AllUnderlayer {
//     pub makepad: MakepadTools,
// }

// impl Display for AllUnderlayer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("🔸 Makepad ToolChains: {}", self.makepad))
//     }
// }

// impl AllUnderlayer {
//     pub fn is_ok(&self) -> bool {
//         self.makepad.is_ok()
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub enum MakepadTools {
    #[default]
    Makepad,
    GenUi,
    Android,
    Ios,
    Wasm,
    Studio,
}

pub struct MakepadState {
    pub makepad: bool,
    pub gen_ui: bool,
    pub android: bool,
    pub ios: bool,
    pub wasm: bool,
    pub studio: bool,
}

impl MakepadState {
    pub fn new(makepad: bool, gen_ui: bool) -> Self {
        Self {
            makepad,
            gen_ui,
            android: false,
            ios: false,
            wasm: false,
            studio: false,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.makepad && self.gen_ui
    }
}

// impl MakepadTools {
//     pub fn is_ok(&self) -> bool {
//         self.makepad && self.gen_ui
//     }
// }

// impl Display for MakepadTools {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let makepad = tag("makepad", self.makepad);
//         let gen_ui = tag("gen-ui", self.gen_ui);

//         f.write_fmt(format_args!("\t\t🔹 {}\n\t\t🔹 {}", makepad, gen_ui))
//     }
// }

// impl From<(bool, bool)> for MakepadTools {
//     fn from((makepad, gen_ui): (bool, bool)) -> Self {
//         Self { makepad, gen_ui }
//     }
// }

// fn tag(name: &str, status: bool) -> colored::ColoredString {
//     return if status {
//         format!("{}: {} ", name, "✅").green()
//     } else {
//         format!("{}: {} ", name, "❌").red()
//     };
// }
