use std::{path::PathBuf, process::Command};

use crate::error::{AppError, AppResult};

pub fn check_prerequisites() -> AppResult<()> {
    let commands = ["gzip", "tar", "mariadb-dump"];

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
