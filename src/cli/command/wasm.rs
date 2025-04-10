use clap::Args;

#[derive(Args, Debug)]
pub struct WasmArgs {
    #[arg(short, long, default_value = None)]
    pub project: Option<String>,
}