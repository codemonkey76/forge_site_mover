// Backup

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use chrono::Utc;

use crate::{
    config::FinalConfig,
    database::DatabaseCredentials,
    error::{AppError, AppResult},
};

pub fn backup_database(creds: &DatabaseCredentials, output_path: &Path) -> AppResult<()> {
    // Prepare temp folder
    if let Some(parent_dir) = output_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    // Start the database backup
    let mut mysqldump = Command::new("mariadb-dump")
        .arg(&creds.database)
        .arg("--no-tablespaces")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::CommandError("mariadb-dump".into(), e))?;

    let mysqldump_stdout = mysqldump.stdout.take().ok_or_else(|| {
        AppError::CommandError(
            "mariadb-dump".into(),
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture mariadb-dump output",
            ),
        )
    })?;

    // Start the gzip command, piping mysqldumps output to gzip
    let mut gzip = Command::new("gzip")
        .arg("-c")
        .stdin(Stdio::from(mysqldump_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::CommandError("gzip".into(), e))?;

    // Connect the `mariadb-dump` output to `gzip` input
    if let Some(mut mysqldump_stdout) = mysqldump.stdout.take() {
        if let Some(gzip_stdin) = gzip.stdin.as_mut() {
            std::io::copy(&mut mysqldump_stdout, gzip_stdin)
                .map_err(|e| AppError::CommandError("gzip".into(), e))?;
        }
    }

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

pub fn backup_files(config: &FinalConfig, output_path: &Path) -> AppResult<()> {
    // Prepare temp folder
    if let Some(parent_dir) = output_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let status = Command::new("tar")
        .current_dir(&config.source_folder)
        .arg("-zcpvf")
        .arg(&output_path)
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| AppError::CommandError("tar".into(), e))?;

    if !status.success() {
        return Err(AppError::CommandError(
            "tar".into(),
            io::Error::new(io::ErrorKind::Other, "tar command failed"),
        ));
    }

    Ok(())
}

pub fn generate_output_path(
    source_folder: &str,
    temp_folder: &str,
    postfix: &str,
) -> Option<PathBuf> {
    let folder_name = Path::new(source_folder).file_name()?.to_string_lossy();

    let date = Utc::now().format("%Y-%m-%d").to_string();

    Some(
        Path::new(temp_folder)
            .join(date)
            .join(format!("{}{}", folder_name, postfix)),
    )
}
