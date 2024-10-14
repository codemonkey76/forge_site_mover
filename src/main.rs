use std::{path::Path, sync::Arc};

use clap::Parser;

use forge_move::{
    args, backup, config,
    error::AppResult,
    feedback,
    forge::{database, site, ForgeClient},
    setup, site_type,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    // Step 1. Check prerequisites.
    setup::check_prerequisites()?;

    // Step 2. Parse config / arguments
    let config = Arc::new(
        config::Config::load()?
            .from_args(args::Args::parse())
            .finalize()?,
    );

    // Step 3. Detect site type
    let site_type = site_type::detect_site_type(Path::new(&config.source_folder))?;

    // Step 4. Backup database
    let mut db_archive = None;
    if let Some(creds) = site_type.get_database_credentials(Path::new(&config.source_folder))? {
        if let Some(output_path) =
            backup::generate_output_path(&config.source_folder, &config.temp_folder, "-db.sql.gz")
        {
            db_archive = Some(output_path.clone());
            feedback::show_spinner(
                move || backup::backup_database(&creds, &output_path),
                "Backing up database",
            )?;
        }
    }

    dbg!(db_archive);

    // Step 5. Backup files
    let mut files_archive = None;
    if let Some(output_path) =
        backup::generate_output_path(&config.source_folder, &config.temp_folder, "-files.tar.gz")
    {
        let config = Arc::clone(&config);
        files_archive = Some(output_path.clone());
        feedback::show_spinner(
            move || backup::backup_files(&config, &output_path),
            "Backing up files",
        )?;
    }

    dbg!(files_archive);

    // Step 6. Create forge site
    let client = Arc::new(ForgeClient::new(&config.forge_api_key)?);

    let csr = site::CreateSiteRequest {
        domain: config.dest_site_name.clone(),
        isolated: config.isolated,
        username: config.user_name.clone().unwrap_or_default(),
        ..Default::default()
    };

    let client_clone = Arc::clone(&client);
    let config_clone = Arc::clone(&config);

    let site = feedback::show_spinner(
        move || client_clone.create_site(&config_clone.dest_server_id, &csr),
        "Creating forge site",
    )?;

    dbg!(site);

    // Step 7. Create destination database
    let cdr = database::CreateDatabaseRequest {
        name: "foo".into(),
        user: "foo".into(),
        password: "password".into(),
    };
    let client_clone = Arc::clone(&client);
    let config_clone = Arc::clone(&config);

    let db = feedback::show_spinner(
        move || client_clone.create_database(&config_clone.dest_server_id, &cdr),
        "Creating forge database",
    )?;

    dbg!(db);

    dbg!(config);
    // use scp to copy files to new server
    // use scp to copy database file
    // restore database on new server

    Ok(())
}
