use uuid::Uuid;
use crate::entity::user_entity::User;
use crate::repository::user_repository::UserRepository;
use sqlx::{query, query_as, sqlite::SqlitePool, Result};

pub struct UserRepositoryImpl {
    pool: SqlitePool,
}

impl UserRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_all_users(&self) -> Result<Vec<User>> {
        query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>> {
        query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_user(&self, user: User) -> Result<User> {
        let id = Uuid::new_v4().to_string();
        query("INSERT INTO users (id, name, email) VALUES (?, ?, ?)")
            .bind(id)
            .bind(user.name)
            .bind(user.email)
            .execute(&self.pool)
            .await?;
        Ok(User { id, ..user.clone() })
    }

    async fn update_user(&self, id: i32, user: User) -> Result<Option<User>> {
        query("UPDATE users SET name = ?, email = ? WHERE id = ?")
            .bind(user.name)
            .bind(user.email)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(Some(user.clone()))
    }

    async fn delete_user(&self, id: i32) -> Result<Option<User>> {
        let user = self.get_user_by_id(id).await?;
        if let Some(user) = &user {
            query("DELETE FROM users WHERE id = ?")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }
        Ok(user)
    }
}