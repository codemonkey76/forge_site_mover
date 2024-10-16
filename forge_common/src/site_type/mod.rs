mod laravel;
mod static_html;
mod wordpress;

use std::path::Path;

pub use laravel::LaravelSite;
pub use static_html::StaticHtmlSite;
pub use wordpress::WordPressSite;

use crate::{
    database::{DatabaseConfigProvider, DatabaseCredentials},
    error::{AppError, AppResult},
};

#[derive(Debug)]
pub enum SiteType {
    Wordpress(WordPressSite),
    Laravel(LaravelSite),
    StaticHtml(StaticHtmlSite),
}

impl SiteType {
    pub fn get_database_credentials(
        &self,
        root_path: &Path,
    ) -> AppResult<Option<DatabaseCredentials>> {
        match self {
            SiteType::Wordpress(site) => site.get_database_credentials(root_path),
            SiteType::Laravel(site) => site.get_database_credentials(root_path),
            SiteType::StaticHtml(site) => site.get_database_credentials(root_path),
        }
    }
}

pub fn detect_site_type(root_path: &Path) -> AppResult<SiteType> {
    if root_path.join("public/wp-config.php").exists() {
        Ok(SiteType::Wordpress(WordPressSite))
    } else if root_path.join(".env").exists() && root_path.join("artisan").exists() {
        Ok(SiteType::Laravel(LaravelSite))
    } else if root_path.join("index.html").exists() {
        Ok(SiteType::StaticHtml(StaticHtmlSite))
    } else {
        Err(AppError::UnknownSiteType(root_path.to_path_buf()))
    }
}
