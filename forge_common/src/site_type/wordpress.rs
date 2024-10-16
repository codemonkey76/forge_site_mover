use std::{fs, path::Path};

use crate::{
    database::{DatabaseConfigProvider, DatabaseCredentials},
    error::{AppError, AppResult},
};

#[derive(Debug)]
pub struct WordPressSite;

impl DatabaseConfigProvider for WordPressSite {
    fn get_database_credentials(&self, root_path: &Path) -> AppResult<Option<DatabaseCredentials>> {
        let config_path = root_path.join("public/wp-config.php");
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| AppError::FileError(config_path.clone(), e))?;

        let username = extract_value(&config_content, "DB_USER")?;
        let password = extract_value(&config_content, "DB_PASSWORD")?;
        let database = extract_value(&config_content, "DB_NAME")?;

        Ok(Some(DatabaseCredentials {
            username,
            password,
            database,
        }))
    }
}

fn extract_value(content: &str, key: &str) -> AppResult<String> {
    let pattern = format!("define\\(\\s*'{}',\\s*'([^']+)'\\s*\\);", key);
    let re = regex::Regex::new(&pattern).unwrap();
    let captures = re
        .captures(content)
        .ok_or(AppError::CredentialParseError(key.to_string()))?;
    Ok(captures[1].to_string())
}
