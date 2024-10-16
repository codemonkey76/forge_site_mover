use std::{fs, path::Path};

use crate::{
    database::{DatabaseConfigProvider, DatabaseCredentials},
    error::{AppError, AppResult},
};

#[derive(Debug)]
pub struct LaravelSite;

impl DatabaseConfigProvider for LaravelSite {
    fn get_database_credentials(&self, root_path: &Path) -> AppResult<Option<DatabaseCredentials>> {
        let config_path = root_path.join(".env");
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| AppError::FileError(config_path.clone(), e))?;

        let username = extract_env_value(&config_content, "DB_USERNAME")?;
        let password = extract_env_value(&config_content, "DB_PASSWORD")?;
        let database = extract_env_value(&config_content, "DB_DATABASE")?;

        Ok(Some(DatabaseCredentials {
            username,
            password,
            database,
        }))
    }
}

fn extract_env_value(content: &str, key: &str) -> AppResult<String> {
    let pattern = format!(r"{}=(.*)", key);
    let re = regex::Regex::new(&pattern).map_err(|e| {
        AppError::RegexParseError(format!("Failed to compile regex for key '{}': {}", key, e))
    })?;

    if let Some(captures) = re.captures(content) {
        if let Some(value) = captures.get(1) {
            return Ok(value.as_str().trim().to_string());
        }
    }

    Err(AppError::CredentialParseError(key.to_string()))
}
