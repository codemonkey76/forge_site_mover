use serde::{Deserialize, Serialize};

use crate::error::AppResult;

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
pub struct CreateSiteResponse {
    pub site: SiteResponse,
}

#[derive(Debug, Deserialize)]
pub struct SiteResponse {
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
    pub fn create_site(
        &self,
        server_id: &str,
        csr: &CreateSiteRequest,
    ) -> AppResult<CreateSiteResponse> {
        self.post_request(server_id, "sites", csr)
    }
}
