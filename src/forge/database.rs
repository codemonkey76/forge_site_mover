use serde::{Deserialize, Serialize};

use crate::error::AppResult;

use super::ForgeClient;

#[derive(Serialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDatabaseResponse {
    pub database: DatabaseResponse,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseResponse {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

impl ForgeClient {
    pub fn create_database(
        &self,
        server_id: &str,
        cdr: &CreateDatabaseRequest,
    ) -> AppResult<CreateDatabaseResponse> {
        self.post_request(server_id, "databases", cdr)
    }

    pub fn delete_database(&self, server_id: &str, database_id: &str) -> AppResult<()> {
        self.delete_request(server_id, "databases", database_id)
    }
}
