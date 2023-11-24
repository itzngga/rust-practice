use crate::model::user_model::{UserRequest, UserResponse};
use sqlx::Result;

#[async_trait::async_trait]
pub trait UserService {
    async fn get_all_users(&self) -> Result<Vec<UserResponse>>;
    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserResponse>>;
    async fn create_user(&self, user: UserRequest) -> Result<UserResponse>;
    async fn update_user(&self, id: &str, user: UserRequest) -> Result<Option<UserResponse>>;
    async fn delete_user(&self, id: &str) -> Result<Option<UserResponse>>;
}