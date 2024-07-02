use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod doc;
mod routers;

use app::config::Config;
use app::state::AppState;
use doc::ApiDoc;
use models::orm::Database;
use routers::create_router;

// TODO: middleware and error handling

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = Config::from_env();
    let conn = Database::connect(&config.db_url)
        .await
        .expect("Database connection failed");

    let app = create_router(AppState { conn })
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind(&config.get_server_url())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
