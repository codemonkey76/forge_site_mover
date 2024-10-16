use std::path::Path;

use crate::error::AppResult;

#[derive(Debug)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub database: String,
}

pub trait DatabaseConfigProvider {
    fn get_database_credentials(&self, root_path: &Path) -> AppResult<Option<DatabaseCredentials>>;
}
