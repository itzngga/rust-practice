use crate::entity::user_entity::User;
use sqlx::Result;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>>;
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>>;
    async fn create_user(&self, user: User) -> Result<User>;
    async fn update_user(&self, id: i32, user: User) -> Result<Option<User>>;
    async fn delete_user(&self, id: i32) -> Result<Option<User>>;
}