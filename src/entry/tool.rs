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
        f.write_str("ℹ Current states:\n")?;
        f.write_fmt(format_args!("  ∙ Basic Tools:\n{}\n", self.basic))?;
        f.write_fmt(format_args!("  ∙ Underlayer Tools:\n{}", self.underlayer))
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
    pub fn is_ok(&self) -> bool {
        self.rustc && self.cargo && self.git
    }
}

impl From<(bool, bool, bool)> for BasicState {
    fn from((rustc, cargo, git): (bool, bool, bool)) -> Self {
        Self { rustc, cargo, git }
    }
}

impl Display for BasicState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\t• rustc: {}\n\t• cargo: {}\n\t• git: {}",
            icon(self.rustc),
            icon(self.cargo),
            icon(self.git)
        ))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BasicTools {
    #[default]
    Ructc,
    Git,
}

#[derive(Debug)]
pub enum UnderlayerTools {
    Makepad(MakepadTools),
}

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

#[allow(unused)]
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

impl Display for MakepadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\t• Makepad: {}\n\t• GenUI: {}",
            icon(self.makepad),
            icon(self.gen_ui),
        ))
    }
}


fn icon(success: bool) -> &'static str {
    if success {
        "✔"
    } else {
        "✘"
    }
}