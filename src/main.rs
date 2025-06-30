use clap::Parser;
use dusty::app::lang;
use dusty::ui::ui_app;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, short)]
    pub lang: Option<String>,
    #[arg(long, short)]
    pub dev: Option<bool>,
    #[arg(long, short)]
    pub test: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let langs = lang::load_langs("spanish");
    if args.dev == Some(true) {
    } else {
        let _ = ui_app::show_ui(langs);
    }
}
