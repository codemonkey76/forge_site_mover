use std::sync::Arc;

use clap::Parser;
use forge_move::{args, config, error::AppResult, forge::ForgeClient};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let config = Arc::new(
        config::Config::load()?
            .from_args(args::Args::parse())
            .finalize()?,
    );

    let client = Arc::new(ForgeClient::new(&config.forge_api_key)?);

    client.delete_user();
    client.delete_database(database_id, server_id;);

    println!("Running reset");
    Ok(())
}
