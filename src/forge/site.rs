use crate::error::AppResult;

use super::ForgeClient;

pub struct SiteData {
    pub domain: String,
    pub project_type: String,
    pub aliases: Vec<String>,
    pub directory: String,
    pub isolated: bool,
    pub username: String,
    pub database: String,
    pub php_version: String,
    pub nginx_template: u32,
}

impl Default for SiteData {
    fn default() -> Self {
        Self {
            domain: "".into(),
            project_type: "php".into(),
            aliases: vec![],
            directory: "".into(),
            isolated: false,
            username: "forge".into(),
            database: "".into(),
            php_version: "php8.1".into(),
            nginx_template: 1,
        }
    }
}

impl ForgeClient {
    pub fn create_site(&self, server_id: &str, site_data: &SiteData) -> AppResult<()> {
        Ok(())
    }
}
