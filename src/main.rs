use clap::Parser;
use theclicker::{Args, TheClicker};

fn main() {
    let args = Args::parse();
    
    let args = match args.load_from_config_or_default() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };
    
    TheClicker::new(args).main_loop();
}
