use crate::model::user_model::{UserRequest, UserResponse};
use crate::service::user_service::UserService;
use tide::{Server, Request, Response, StatusCode};
use crate::service::user_service_impl::UserServiceImpl;

#[derive(Clone)]
pub struct UserController<T: UserService> {
    service: T,
}

impl<T: UserService> UserController<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }

    pub fn setup_routes(mut self, mut server: tide::Server<()>) {
        server.at("/users").get(self.get_all_users());
        server.at("/users/:id").get(self.get_user_by_id);
        server.at("/users").post(self.create_user);
        server.at("/users/:id").patch(self.update_user);
        server.at("/users/:id").delete(self.delete_user);
    }

    pub async fn get_all_users(&self, _req: Request<()>) -> tide::Result {
        let users = self.service.get_all_users().await?;
        let json = serde_json::to_string(&users)
            .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
        Ok(Response::builder(StatusCode::Ok)
            .body(json)
            .content_type(tide::http::mime::JSON)
            .build())
    }

    pub async fn get_user_by_id(&self,req: Request<()>) -> tide::Result<Response> {
        let id: &str = req.param("id")?;
        let user = self.service.get_user_by_id(id).await?;
        match user {
            Some(user) => {
                let json = serde_json::to_string(&user)
                    .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
                Ok(Response::builder(StatusCode::Ok)
                    .body(json)
                    .content_type(tide::http::mime::JSON)
                    .build())
            }
            None => Ok(Response::builder(StatusCode::NotFound).build()),
        }
    }

    pub async fn create_user(&self, req: Request<()>) -> tide::Result<Response> {
        let user: UserRequest = req.body_json().await?;
        let created_user = self.service.create_user(user).await?;
        let json = serde_json::to_string(&created_user)
            .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
        Ok(Response::builder(StatusCode::Created)
            .body(json)
            .content_type(tide::http::mime::JSON)
            .build())
    }

    pub async fn update_user(&self, req Request<()>) -> tide::Result<Response> {
        let id = req.param("id")?.to_string();
        let user: UserRequest = req.clone().body_json().await?;
        let updated_user = self.service.update_user(&id, user).await?;
        match updated_user {
            Some(user) => {
                let json = serde_json::to_string(&user)
                    .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
                Ok(Response::builder(StatusCode::Ok)
                    .body(json)
                    .content_type(tide::http::mime::JSON)
                    .build())
            }
            None => Ok(Response::builder(StatusCode::NotFound).build()),
        }
    }

    pub async fn delete_user(&self, req: Request<()>) -> tide::Result<Response> {
        let id: &str = req.param("id")?;
        let deleted_user = self.service.delete_user(id).await?;
        match deleted_user {
            Some(user) => {
                let json = serde_json::to_string(&user)
                    .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
                Ok(Response::builder(StatusCode::Ok)
                    .body(json)
                    .content_type(tide::http::mime::JSON)
                    .build())
            }
            None => Ok(Response::builder(StatusCode::NotFound).build()),
        }
    }
}
