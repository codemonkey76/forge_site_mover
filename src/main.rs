use std::{
    fs::File,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use args::Args;
use clap::Parser;
use config::Config;
use error::{AppError, AppResult};

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
    check_prerequisites()?;

    let config = Config::load()?.from_args(Args::parse()).finalize();

    println!("Config: {:?}", config);

    Ok(())
}

fn check_prerequisites() -> AppResult<()> {
    let commands = ["gzip", "tar", "mysqldump"];

    for cmd in &commands {
        let output = Command::new("which")
            .arg(cmd)
            .output()
            .map_err(|e| AppError::FileError(PathBuf::from(cmd), e))?;

        if !output.status.success() {
            return Err(AppError::MissingPrerequisites(cmd.to_string()));
        }
    }

    Ok(())
}

fn backup_database(db_name: &str, output_path: &Path) -> AppResult<()> {
    // Start the database backup
    let mysqldump = Command::new("mysqldump")
        .arg(db_name)
        .arg("--no-tablespaces")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::CommandError("mysqldump".into(), e))?;

    // Start the gzip command, piping mysqldumps output to gzip
    let mut gzip = Command::new("gzip")
        .arg("-c")
        .stdin(mysqldump.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::CommandError("gzip".into(), e))?;

    // Create the output file and write the gzip output to it
    let mut output_file =
        File::create(output_path).map_err(|e| AppError::FileError(output_path.to_path_buf(), e))?;

    if let Some(mut gzip_stdout) = gzip.stdout.take() {
        std::io::copy(&mut gzip_stdout, &mut output_file)
            .map_err(|e| AppError::FileError(output_path.to_path_buf(), e))?;
    }

    // Wait for the commands to complete
    gzip.wait()
        .map_err(|e| AppError::CommandError("gzip".into(), e))?;

    Ok(())
}
