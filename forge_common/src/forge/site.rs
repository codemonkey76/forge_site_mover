use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

use super::ForgeClient;

#[derive(Debug, Serialize)]
pub struct CreateSiteRequest {
    pub domain: String,
    pub project_type: String,
    pub aliases: Vec<String>,
    pub directory: String,
    pub isolated: bool,
    pub username: String,
    pub database: String,
    pub php_version: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteResponse {
    pub site: Site,
}

#[derive(Debug, Deserialize)]
pub struct ListSiteResponse {
    pub sites: Vec<Site>,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub id: u32,
    pub server_id: u32,
    pub name: String,
    pub aliases: Vec<String>,
    pub directory: String,
    pub wildcards: bool,
    pub status: String,
    pub repository: Option<String>,
    pub repository_provider: Option<String>,
    pub repository_branch: Option<String>,
    pub repository_status: Option<String>,
    pub quick_deploy: bool,
    pub project_type: String,
    pub php_version: String,
    pub app: Option<String>,
    pub app_status: Option<String>,
    pub slack_channel: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub telegram_chat_title: Option<String>,
    pub teams_webhook_url: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub created_at: String,
    pub telegram_secret: String,
    pub username: String,
    pub deployment_url: String,
    pub is_secured: bool,
    pub web_directory: String,
    pub isolated: Option<bool>,
    pub tags: Option<Vec<String>>,
}

impl Default for CreateSiteRequest {
    fn default() -> Self {
        Self {
            domain: "".into(),
            project_type: "php".into(),
            aliases: vec![],
            directory: "".into(),
            isolated: false,
            username: "forge".into(),
            database: "".into(),
            php_version: "php83".into(),
        }
    }
}

impl ForgeClient {
    pub fn wait_for_site_ready(&self, server_id: &str, site_id: &str) -> AppResult<()> {
        let max_attempts = 10;
        let delay = Duration::from_secs(10);

        for attempt in 1..=max_attempts {
            let site_status = self.get_site_status(server_id, site_id)?;

            if site_status == "installed" {
                return Ok(());
            }
            thread::sleep(delay);
        }

        Err(AppError::ForgeAPIError(
            "Site is taking too long to reach 'ready' state in the expected timeframe".into(),
        ))
    }
    pub fn get_site_status(&self, server_id: &str, site_id: &str) -> AppResult<String> {
        let response = self.get_site(server_id, site_id)?;

        Ok(response.site.status)
    }
    pub fn create_site(&self, server_id: &str, csr: &CreateSiteRequest) -> AppResult<SiteResponse> {
        self.post_request(server_id, "sites", csr)
    }

    pub fn list_sites(&self, server_id: &str) -> AppResult<ListSiteResponse> {
        self.list_request(server_id, "sites")
    }

    pub fn get_site(&self, server_id: &str, site_id: &str) -> AppResult<SiteResponse> {
        self.get_request(server_id, "sites", site_id)
    }

    pub fn delete_site(&self, server_id: &str, site_id: &str) -> AppResult<()> {
        self.delete_request(server_id, "sites", site_id)
    }

    pub fn delete_site_by_name(&self, server_id: &str, site_name: &str) -> AppResult<()> {
        match self
            .list_sites(server_id)?
            .sites
            .iter()
            .find(|site| site.name == site_name)
            .map(|site| site.id)
        {
            Some(site_id) => self.delete_site(server_id, &site_id.to_string()),
            None => Err(AppError::ForgeAPIError(format!(
                "Could not find site with the name: {}",
                site_name
            ))),
        }
    }
}
