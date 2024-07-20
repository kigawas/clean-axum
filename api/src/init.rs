use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use app::config::Config;
use app::state::AppState;

use crate::routers::create_router;

// TODO: middleware, logging, authentication
pub fn setup_router(conn: DatabaseConnection) -> Router {
    create_router(AppState { conn })
}

pub fn setup_config() -> Config {
    dotenvy::dotenv().ok();
    Config::from_env()
}

pub async fn setup_db(db_url: &str) -> DatabaseConnection {
    let opt = ConnectOptions::new(db_url);
    Database::connect(opt)
        .await
        .expect("Database connection failed")
}
