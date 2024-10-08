use args::Args;
use clap::Parser;
use config::Config;
use error::AppResult;

mod args;
mod config;
mod error;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    // Step 1. Load config
    let config = Config::load()?.from_args(Args::parse()).finalize();

    println!("Config: {:?}", config);
    // Step 2: Process command-line arguments
    //    let args = c

    Ok(())
}
