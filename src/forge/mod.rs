pub mod database;
pub mod site;
pub mod user;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
};

use crate::error::{AppError, AppResult};

pub struct ForgeClient {
    api_key: String,
    base_url: String,
    client: Client,
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

        Ok(Self {
            api_key: api_key.to_string(),
            base_url: "https://forge.laravel.com/api/v1".into(),
            client: Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| AppError::ForgeAPIError("Unable to contruct request client".into()))?,
        })
    }
}
