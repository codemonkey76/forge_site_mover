use std::path::Path;

use crate::{
    database::{DatabaseConfigProvider, DatabaseCredentials},
    error::AppResult,
};

#[derive(Debug)]
pub struct StaticHtmlSite;

impl DatabaseConfigProvider for StaticHtmlSite {
    fn get_database_credentials(
        &self,
        _root_path: &Path,
    ) -> AppResult<Option<DatabaseCredentials>> {
        Ok(None)
    }
}
