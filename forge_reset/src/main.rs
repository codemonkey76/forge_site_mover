use std::sync::Arc;

use clap::Parser;
use forge_common::{args, config, error::AppResult, feedback, forge::ForgeClient};

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

    let client_clone = client.clone();
    let config_clone = config.clone();

    feedback::show_spinner(
        move || {
            client_clone
                .delete_site_by_name(&config_clone.dest_server_id, &config_clone.dest_site_name)
        },
        "Deleting forge site",
    )?;

    let client_clone = client.clone();
    let config_clone = config.clone();
    feedback::show_spinner(
        move || {
            client_clone
                .delete_database_by_name(&config_clone.dest_server_id, &config_clone.dest_db)
        },
        "Deleting forge database",
    )?;

    let client_clone = client.clone();
    let config_clone = config.clone();

    let user_name = config_clone.user_name.clone();

    if let Some(user) = user_name {
        feedback::show_spinner(
            move || client_clone.delete_user_by_name(&config_clone.dest_server_id, &user),
            "Deleting database user",
        )?;
    }

    Ok(())
}
