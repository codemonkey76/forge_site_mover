use std::path::Path;

use clap::Parser;

use forge_move::{
    args, backup, config,
    error::AppResult,
    feedback,
    forge::{
        site::{self, SiteData},
        ForgeClient,
    },
    setup, site_type,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    setup::check_prerequisites()?;

    let config = config::Config::load()?
        .from_args(args::Args::parse())
        .finalize()?;

    let client = ForgeClient::new(&config.forge_api_key)?;

    let site_type = site_type::detect_site_type(Path::new(&config.source_folder))?;

    if let Some(creds) = site_type.get_database_credentials(Path::new(&config.source_folder))? {
        if let Some(output_path) =
            backup::generate_output_path(&config.source_folder, &config.temp_folder, "-db.sql.gz")
        {
            feedback::show_spinner(
                move || backup::backup_database(&creds, &output_path),
                "Backing up database",
            );
        }
    }

    if let Some(output_path) =
        backup::generate_output_path(&config.source_folder, &config.temp_folder, "-files.tar.gz")
    {
        feedback::show_spinner(
            move || backup::backup_files(&config, &output_path),
            "Backing up files",
        );
    }

    client.create_site("foo.com.au", &SiteData::default())?;

    // Create site
    // Create database
    // use scp to copy files to new server
    // use scp to copy database file
    // restore database on new server

    Ok(())
}
