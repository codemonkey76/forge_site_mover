use core::fmt;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ConfigReadError(PathBuf, io::Error),
    ConfigParseError(PathBuf, toml::de::Error),
    ConfigSerializationError(PathBuf, toml::ser::Error),
    FileError(PathBuf, io::Error),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ConfigReadError(path, err) => {
                write!(
                    f,
                    "Failed to read config file at {}: {}",
                    path.display(),
                    err
                )
            }
            AppError::ConfigParseError(path, err) => {
                write!(
                    f,
                    "Failed to parse config file at {}: {}",
                    path.display(),
                    err
                )
            }
            AppError::ConfigSerializationError(path, err) => {
                write!(
                    f,
                    "Failed to serialize config file at {}: {}",
                    path.display(),
                    err
                )
            }
            AppError::FileError(path, err) => {
                write!(f, "File error at {}: {}", path.display(), err)
            }
            AppError::DatabaseError(message) => {
                write!(f, "Database error: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ConfigReadError(_, source) => Some(source),
            AppError::ConfigParseError(_, source) => Some(source),
            AppError::ConfigSerializationError(_, source) => Some(source),
            AppError::FileError(_, source) => Some(source),
            AppError::DatabaseError(_) => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::FileError(PathBuf::new(), error)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(error: toml::de::Error) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

fn load_config(path: &Path) -> Result<String, AppError> {
    let config_content =
        fs::read_to_string(path).map_err(|e| AppError::ConfigReadError(path.to_path_buf(), e))?;
    Ok(config_content)
}
