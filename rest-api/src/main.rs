use dotenv::dotenv;

mod config;
mod controller;
mod entity;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    // Setup database connection
    let pool = config::sqlite::establish_db_connection().await?;

    // Initialize services and repositories
    let user_repository = repository::user_repository_impl::UserRepositoryImpl::new(pool.clone());
    let user_service = service::user_service_impl::UserServiceImpl::new(Box::new(user_repository));
    let user_controller = controller::user_controller::UserController::new(user_service);

    // Create Tide app with state
    let mut app = tide::with_state(user_controller);

    // Initialize user controller
    user_controller.setup_routes(&mut app);

    // Start the server
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}