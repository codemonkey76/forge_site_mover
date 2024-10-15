use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Parser;

use forge_move::{
    args, backup, config,
    error::AppResult,
    feedback,
    forge::{database, site, ForgeClient},
    setup, site_type,
};
use rand::Rng;

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
    let mut db_archive: Option<PathBuf> = None;
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

    //dbg!(&db_archive);

    // Step 5. Backup files
    let mut files_archive: Option<PathBuf> = None;
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

    //dbg!(&files_archive);

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

    ////dbg!(&site);
    let client_clone = Arc::clone(&client);
    let config_clone = Arc::clone(&config);

    // Step 7. Create destination database
    let password = generate_password(20);
    dbg!(&password);

    let cdr = database::CreateDatabaseRequest {
        name: config_clone.dest_db.clone(),
        user: config_clone.user_name.clone().unwrap_or("forge".into()),
        password: password.clone(),
    };

    feedback::show_spinner(
        move || client_clone.create_database(&config_clone.dest_server_id, &cdr),
        "Creating forge database",
    )?;

    //dbg!(&db);
    let web_directory = site.site.web_directory.clone();
    //let web_directory = format!("/home/foo/foo.com.au");
    //let password = format!("o3zc7s5gxtw9fjl2pcv2");
    //let files_archive = Some(Path::new(
    //    "/tmp/forge-move/2024-10-15/callcenter-files.tar.gz",
    //));
    //let db_archive = Some(Path::new("/tmp/forge-move/2024-10-15/callcenter-db.sql.gz"));

    dbg!(&web_directory);
    //dbg!(&config);
    // Step 8. Restore files to target server
    if let Some(ref archive) = files_archive {
        let archive_clone = archive.clone();
        let config_clone = config.clone();
        feedback::show_spinner(
            move || {
                backup::restore_files(
                    archive_clone
                        .to_str()
                        .expect("Failed to convert PathBuf to string"),
                    &config_clone.dest_host,
                    config_clone.user_name.clone(),
                    &web_directory,
                )
            },
            "Copying files via SSH",
        )?;
    }

    if let Some(ref archive) = db_archive {
        let archive_clone = archive.clone();
        feedback::show_spinner(
            move || {
                backup::restore_database(
                    archive_clone
                        .to_str()
                        .expect("Failed to convert PathBuf to string"),
                    &config.dest_host,
                    config.user_name.clone(),
                    &config.dest_db,
                    &password,
                )
            },
            "Restoring DB on destination server",
        )?;
    }

    Ok(())
}

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
