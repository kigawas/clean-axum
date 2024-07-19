use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use app::config::Config;
use app::state::AppState;

use crate::doc::ApiDoc;
use crate::routers::create_router;

pub fn setup_router(conn: DatabaseConnection) -> Router {
    create_router(AppState { conn })
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
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
