pub mod database;
pub mod site;
pub mod user;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{AppError, AppResult};

pub struct ForgeClient {
    pub api_key: String,
    pub base_url: String,
    pub version: String,
    pub client: Client,
}

impl ForgeClient {
    pub fn new(api_key: &str) -> AppResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(|_| {
                AppError::ForgeAPIError("Unable to construct Authorization string".into())
            })?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(Self {
            api_key: api_key.to_string(),
            base_url: "https://forge.laravel.com".into(),
            version: "v1".into(),
            client: Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| AppError::ForgeAPIError("Unable to contruct request client".into()))?,
        })
    }
    pub fn post_request<T: DeserializeOwned, U: Serialize>(
        &self,
        server_id: &str,
        endpoint: &str,
        request_data: &U,
    ) -> AppResult<T> {
        let url = format!(
            "{}/api/{}/servers/{}/{}",
            self.base_url, self.version, server_id, endpoint
        );

        let response = self
            .client
            .post(&url)
            .json(request_data)
            .send()
            .map_err(|e| AppError::ForgeAPIError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "No error details available".into());

            return Err(AppError::ForgeAPIError(format!(
                "Request return error {}: {}",
                status, error_text
            )));
        }

        let response_text = response
            .text()
            .map_err(|e| AppError::ForgeAPIError(format!("Failed to read response text: {}", e)))?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ForgeAPIError(format!("Failed to parse response: {}", e)))?;

        // println!("Site created with ID: {}", create_site_response.site.id);

        Ok(parsed_response)
    }
}
