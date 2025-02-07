pub const DEFAULT_GITIGNORE: &str = r#"# DEFAULT GITIGNORE CREATE BY `RACT`
target/
Cargo.lock
**/*.DS_Store
"#;

pub const MAKEPAD_LIB_RS: &str = r#"pub use makepad_widgets;
pub mod app;
"#;

pub const MAKEPAD_MAIN_RS: &str = r#"// This is a simple makepad example
fn main(){
    ${project_name}::app::app_main()
}
"#;

pub const MAKEPAD_APP_RS: &str = r#"// App.rs
use makepad_widgets::*;

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <ScrollXYView>{
                    flow: Down,
                    spacing: 16,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg:{
                        fn pixel(self) -> vec4 {
                            return #2A2E37;
                        }
                    }
                    <Label> {
                        text: "This is a simple Makepad example!"
                        draw_text:{
                            color:#fff
                            text_style: {font_size: 24.0}
                        }
                    }
                    label1 = <Label> {
                        text: "Counter:"
                        draw_text:{
                            color:#fff
                            text_style: {font_size: 12.0}
                        }
                    }
                    button1 = <Button> {
                        text: "Click me!",
                        padding: {left: 14.0, right: 14.0, top: 8.0, bottom: 8.0},
                        draw_text:{color:#fff}
                    }
                }
            }
        }
    }
}  

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] counter: usize,
 }
 
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        if self.ui.button(id!(button1)).clicked(&actions) {
            self.counter += 1;
            self.ui.label(id!(label1)).set_text_and_redraw(cx, &format!("Counter: {}", self.counter));
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App); 
"#;

pub const LOGO: &str = r#"
                                                      
     _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/   
  _/        _/        _/_/    _/  _/    _/    _/      
 _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/       
_/    _/  _/        _/    _/_/  _/    _/    _/        
 _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/       
                                                      
"#;

pub const VIEW_MOD_GEN: &str = r#"<script>
pub mod root;
pub mod home;
</script>"#;

pub const ROOT_GEN: &str = r###"// default template for root.gen
<template>
  <component name="UiRoot">
    <view theme="Dark" height="Fill" width="Fill" align="0.5" spacing="12.0" flow="Down">
      <label text="'This is an easy GenUI template'" font_size="36.0"></label>
      <EasyLabel></EasyLabel>
      <Home></Home>
    </view>
  </component>
</template>

<script>
import!{
  crate::views::home::*;
  crate::components::easy::*;
}
</script>
"###;

pub const HOME_GEN: &str = r###"<template>
    <component name="Home" class="home_view">
        <Hello id="header"></Hello>
        <button id="my_btn" theme="Error" @clicked="click_btn()">
            <label as_prop="slot" text="'Click Me!'" font_size="12.0"></label>
        </button>
    </component>
</template>

<script>
import!{
    crate::components::hello::*;
}

#[prop]
pub struct Home{
    num: u32
}

// init instance prop
let mut prop = default_prop!{
    Home{
        num: 0
    }
};

fn click_btn(){
    let mut num = prop.get_num();
    num += 1;
    prop.set_num(num);
    // use c_ref! you can get component ref
    let header = c_ref!(header);
    let num = prop.get_num();
    header.set_my_text(format!("Clicked: {}", num));
}
</script>

<style>
.home_view{
    height: Fit;
    flow: Down;
    align: {
        x: 0.5,
        y: 0.5
    };
    spacing: 16.0;
    padding: 12.0;
}
</style>
"###;

pub const HELLO_GEN: &str = r###"<template>
    <component name="Hello" class="hello_view">
        <label id="my_lb" :text="my_text" font_size="16.0"></label>
    </component>
</template>

<script>
#[prop]
pub struct Hello{
    my_text: String
}

let prop = default_prop!{
    Hello{
        my_text: "Clicked: 0".to_string()
    }
};
</script>

<style>
.hello_view{
    height: Fit;
    width: Fill;
    align: {
        x: 0.5, 
        y: 0.5
    };
}
</style>
"###;

pub const COMPONENT_MOD_GEN: &str = r#"<script>
pub mod hello;
pub mod easy;
</script>"#;

pub const EASY_GEN: &str = r###"<template>
    <view id="EasyLabel" height="Fit" align="{x: 0.5}">
        <label text="'You now using Makepad + GenUI + GenUI Components'" font_size="16.0" color="#FF7043"></label>
    </view>
</template>
"###;

pub const GENUI_README: &str = r#"# GenUI Project

Welcome to the GenUI Project! This project is built with the Ract tool and serves as a standardized GenUI project template. Below is the version information for the main frameworks and tools used:

| Framework/Tool                | Version/Branch        |
|-------------------------------|-----------------------|
| **GenUI**                     | v0.1.0                |
| **Makepad**                   | gen_ui (branch)       |
| **Ract**                      | v0.1.0                |
| **GenUI Built-in Component**  | v0.2.0                |

---

## Project Structure

The GenUI project adopts a typical Rust Workspace structure, consisting of multiple sub-projects. Below is a description of the project directory and its functions:

> [!TIP]
> The following content symbols are explained:
> - `#`: Descriptor, the specific name is unknown, for example, `#workspace` means the project name of a workspace project created by the user
> - `[]`: means optional

```
#workspace
│
├── source_project             // Source code package
│   ├── src/                   // Rust source code files
│   │   └── main.rs            // Project entry file (usually empty)
│   ├── resources/             // Static resources
│   ├── views/                 // Main page files
│   │   ├── root.gen           // UI entry file
│   │   ├── home.gen           // Home page
│   │   └── mod.gen            // Page export mod file
│   ├── components/            // Component files
│   │   ├── hello.gen          // Hello component
│   │   ├── easy.gen           // Easy component
│   │   └── mod.gen            // Component export mod file
│   ├── .gen_ui_cache/         // Cache files
│   ├── Cargo.toml             // Rust package configuration file
│   └── gen_ui.toml            // GenUI project configuration file
│
├── compiled_project           // Compiled result package
│
├── .ract                      // Ract configuration file
├── Cargo.toml                 // Workspace configuration file
├── Cargo.lock                 // Dependency lock file
├── [.gitignore]               // gitignore (optional)
└── [LICENSE]                  // Project license file (optional)
```

---

## Launch and Compilation

Managed by the Ract tool, you can easily compile and launch the project:

1. Navigate to the project root directory in the terminal:
```bash
cd /path/to/workspace
```
2. Run the following command to start the project:
```bash
ract run
```
Ract will automatically locate the source code directory and start the project. Ensure that the .ract and gen_ui.toml configuration files are correctly set up. Upon successful launch, you will see the following output:

```
🥳 Welcome to use ract project runner!

🔸 Now you can run makepad and gen_ui (Comming Soon) projects
❗️ Please make sure your project root has a `.ract` file to indicate the project type
🔸 If you are unfamiliar with the `.ract` file, please run `ract book` to search (Coming Soon)


                                                      
     _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/   
  _/        _/        _/_/    _/  _/    _/    _/      
 _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/       
_/    _/  _/        _/    _/_/  _/    _/    _/        
 _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/       
                                                      

GenUI-Compiler :: [2025-01-20 06:17:31] :: INFO >>> 🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!
    Creating binary (application) `src_gen_0` package
note: see more `Cargo.toml` keys and their definitions at <https://doc.rust-lang.org/cargo/reference/manifest.html> 
GenUI-Compiler :: [2025-01-20 06:17:31] :: INFO >>> 🔧 Watcher Service started successfully! Ract is watching: `/User/GenUI/Source/Project/Path`
```
## Learning GenUI
For more information, please see: [GenUI Official Documentation](https://privoce.github.io/GenUI.github.io/).
## Collaboration and Feedback
> [!IMPORTANT]
> GenUI is currently in the early stages of development, with many features still being planned and implemented. We welcome community feedback and collaboration! If you have any suggestions for the framework, need to report an issue, or would like to add features, please contact us through the following channels:

- **GitHub**: [https://github.com/Privoce/GenUI](https://github.com/Privoce/GenUI)
- **Discord**: [https://discord.gg/jVEJDhE75Y](https://discord.gg/jVEJDhE75Y)
- **Email**: [syf20020816@outlook.com](mailto:syf20020816@outlook.com)
- **Collaboration Email**: [han@privoce.com](mailto:han@privoce.com)


Thank you for your support, and we look forward to building a better GenUI with you!
"#;