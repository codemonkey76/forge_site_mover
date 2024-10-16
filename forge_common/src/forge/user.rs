use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

use super::ForgeClient;

#[derive(Debug, Serialize)]
pub struct CreateUserRequest {
    name: String,
    password: String,
    databases: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    user: User,
}

#[derive(Debug, Deserialize)]
pub struct ListUserResponse {
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    status: String,
    created_at: String,
    databases: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserRequest {
    databases: Vec<u32>,
}

impl ForgeClient {
    pub fn create_user(&self, server_id: &str, cur: &CreateUserRequest) -> AppResult<UserResponse> {
        self.post_request(server_id, "database-users", cur)
    }
    pub fn list_users(&self, server_id: &str) -> AppResult<ListUserResponse> {
        self.list_request(server_id, "database-users")
    }
    pub fn get_user(&self, server_id: &str, user_id: &str) -> AppResult<UserResponse> {
        self.get_request(server_id, "database-users", user_id)
    }
    pub fn update_user(
        &self,
        server_id: &str,
        user_id: &str,
        uur: &UpdateUserRequest,
    ) -> AppResult<UserResponse> {
        self.put_request(server_id, "database-users", user_id, uur)
    }
    pub fn delete_user(&self, server_id: &str, user_id: &str) -> AppResult<()> {
        self.delete_request(server_id, "database-users", user_id)
    }
    pub fn delete_user_by_name(&self, server_id: &str, user_name: &str) -> AppResult<()> {
        match self
            .list_users(server_id)?
            .users
            .iter()
            .find(|user| user.name == user_name)
            .map(|user| user.id)
        {
            Some(user_id) => self.delete_user(server_id, &user_id.to_string()),
            None => Err(AppError::ForgeAPIError(format!(
                "Could not find user with the name: {}",
                user_name
            ))),
        }
    }
}
