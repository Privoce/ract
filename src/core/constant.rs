pub const DEFAULT_GITIGNORE: &str = r#"
target/
Cargo.lock
**/*.DS_Store
"#;

pub const DEFAULT_ENV_TOML: &str = r#"
[dependencies]
makepad-widgets = "${makepad-widgets}"
gen_components = "${gen_components}"
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