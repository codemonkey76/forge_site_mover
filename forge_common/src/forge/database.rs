use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

use super::ForgeClient;

#[derive(Serialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseResponse {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct ListDatabaseResponse {
    pub databases: Vec<Database>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
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
    ) -> AppResult<DatabaseResponse> {
        self.post_request(server_id, "databases", cdr)
    }

    pub fn list_databases(&self, server_id: &str) -> AppResult<ListDatabaseResponse> {
        self.list_request(server_id, "databases")
    }

    pub fn get_database(&self, server_id: &str, database_id: &str) -> AppResult<DatabaseResponse> {
        self.get_request(server_id, "databases", database_id)
    }

    pub fn delete_database(&self, server_id: &str, database_id: &str) -> AppResult<()> {
        self.delete_request(server_id, "databases", database_id)
    }

    pub fn delete_database_by_name(&self, server_id: &str, database_name: &str) -> AppResult<()> {
        match self
            .list_databases(server_id)?
            .databases
            .iter()
            .find(|database| database.name == database_name)
            .map(|database| database.id)
        {
            Some(database_id) => self.delete_database(server_id, &database_id.to_string()),
            None => Err(AppError::ForgeAPIError(format!(
                "Could not find database with the name: {}",
                database_name
            ))),
        }
    }
}
