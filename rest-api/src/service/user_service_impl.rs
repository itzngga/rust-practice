use sqlx::Result;
use crate::model::user_model::{UserRequest, UserResponse};
use crate::repository::user_repository::UserRepository;
use crate::repository::user_repository_impl::UserRepositoryImpl;
use crate::service::user_service::UserService;

pub struct UserServiceImpl {
    user_repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn get_all_users(&self) -> Result<Vec<UserResponse>> {
        let users = self.repository.get_all_users().await?;
        // Convert to UserResponse and return
        let response = users.iter().map(|user| user.into()).collect();
        Ok(response)
    }

    async fn get_user_by_id(&self, id: &str) -> Result<Option<UserResponse>> {
        let id = id.parse::<i32>().unwrap();
        let user = self.repository.get_user_by_id(id).await?;
        // Convert to Optional UserResponse and return
        let response = user.map(|user| user.into());
        Ok(response)
    }

    async fn create_user(&self, user: UserRequest) -> Result<UserResponse> {
        // Convert UserRequest to User and create in repository
        let user = user.into();
        let created_user = self.repository.create_user(user).await?;
        // Convert to UserResponse and return
        let response: UserResponse = created_user.into();
        Ok(response)
    }

    async fn update_user(&self, id: &str, user: UserRequest) -> Result<Option<UserResponse>> {
        // Convert UserRequest to User and update in repository
        let user = user.into();
        let id = id.parse::<i32>().unwrap();
        let updated_user = self.repository.update_user(id, user).await?;
        // Convert to Optional UserResponse and return
        let response = updated_user.map(|user| user.into());
        Ok(response)
    }

    async fn delete_user(&self, id: &str) -> Result<Option<UserResponse>> {
        let id = id.parse::<i32>().unwrap();
        let deleted_user = self.repository.delete_user(id).await?;
        // Convert to Optional UserResponse and return
        let response = deleted_user.map(|user| user.into());
        Ok(response)
    }
}
