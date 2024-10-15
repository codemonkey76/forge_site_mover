pub mod database;
pub mod site;
pub mod user;

use reqwest::{
    blocking::{Client, RequestBuilder},
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

        let request_builder = self.client.post(&url).json(request_data);
        self.send_request(request_builder)
    }

    fn delete_request<T: DeserializeOwned>(
        &self,
        server_id: &str,
        endpoint: &str,
        resource_id: &str,
    ) -> AppResult<T> {
        let url = format!(
            "{}/api/{}/servers/{}/{}/{}",
            self.base_url, self.version, server_id, endpoint, resource_id
        );

        let request_builder = self.client.delete(&url);
        self.send_request(request_builder)
    }

    fn get_request<T: DeserializeOwned>(&self, server_id: &str, endpoint: &str) -> AppResult<T> {
        let url = format!(
            "{}/api/{}/servers/{}/{}",
            self.base_url, self.version, server_id, endpoint
        );

        let request_builder = self.client.get(&url);
        self.send_request(request_builder)
    }

    fn send_request<T: DeserializeOwned>(&self, request_builder: RequestBuilder) -> AppResult<T> {
        let response = request_builder
            .send()
            .map_err(|e| AppError::ForgeAPIError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "No error details available".into());

            return Err(AppError::ForgeAPIError(format!(
                "Request returned error {}: {}",
                status, error_text
            )));
        }

        let response_text = response
            .text()
            .map_err(|e| AppError::ForgeAPIError(format!("Failed to read response text: {}", e)))?;

        // Uncomment this line for debugging the raw response text
        dbg!(&response_text);

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ForgeAPIError(format!("Failed to parse response: {}", e)))?;

        Ok(parsed_response)
    }
}
