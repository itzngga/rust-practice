use serde;

#[derive(Debug, serde::Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}