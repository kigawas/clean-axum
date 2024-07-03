mod db;
mod utils;

use db::migrate;
use utils::create_dev_db;

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() {
    println!("Starting with tokio");

    tracing_subscriber::fmt::init();

    let config = api::setup_config();
    create_dev_db(&config.db_url);

    let conn = api::setup_db(&config.db_url).await;
    migrate(&conn).await.expect("Migration failed!");

    let router = api::setup_router(conn);
    let listener = tokio::net::TcpListener::bind(&config.get_server_url())
        .await
        .unwrap();
    api::axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    println!("Starting with shuttle");

    let db_url = secrets.get("DATABASE_URL").expect("secret was not found");
    create_dev_db(&db_url);

    let conn = api::setup_db(&db_url).await;
    migrate(&conn).await.expect("Migration failed!");

    let router = api::setup_router(conn);
    Ok(router.into())
}
