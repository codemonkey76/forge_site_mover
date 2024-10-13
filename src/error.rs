use core::fmt;
use std::{io, path::PathBuf};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ConfigReadError(PathBuf, io::Error),
    ConfigSerializationError(PathBuf, toml::ser::Error),
    InputError(dialoguer::Error),
    FileError(PathBuf, io::Error),
    DatabaseError(String),
    MissingPrerequisites(String),
    CommandError(String, io::Error),
    UnknownSiteType(PathBuf),
    CredentialParseError(String),
    ForgeAPIError(String),
    RegexParseError(String),
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
            AppError::InputError(err) => {
                write!(f, "Input error: {}", err)
            }
            AppError::CommandError(cmd, err) => {
                write!(f, "Command `{}` failed to execute: {}", cmd, err)
            }
            AppError::MissingPrerequisites(cmd) => {
                write!(f, "Missing prerequisites: {}", cmd)
            }
            AppError::UnknownSiteType(path) => {
                write!(f, "Unknown site type at path: {}", path.display())
            }
            AppError::CredentialParseError(key) => {
                write!(f, "Unable to parse credentials at path: {}", key)
            }
            AppError::RegexParseError(message) => {
                write!(f, "Regex: {}", message)
            }
            AppError::ForgeAPIError(message) => {
                write!(f, "Forge API: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ConfigReadError(_, source) => Some(source),
            AppError::ConfigSerializationError(_, source) => Some(source),
            AppError::FileError(_, source) => Some(source),
            AppError::CommandError(_, source) => Some(source),
            AppError::DatabaseError(_) => None,
            AppError::InputError(source) => Some(source),
            AppError::MissingPrerequisites(_) => None,
            AppError::ForgeAPIError(_) => None,
            AppError::UnknownSiteType(_) => None,
            AppError::CredentialParseError(_) => None,
            AppError::RegexParseError(_) => None,
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
