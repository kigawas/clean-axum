use utils::migrate;

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    use utils::create_dev_db;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,clean_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting with tokio");

    let config = api::setup_config();
    create_dev_db(&config.db_url);

    let conn = api::setup_db(&config.db_url).await;
    migrate(&conn).await.expect("Migration failed!");

    let router = api::setup_router(conn);
    let listener = tokio::net::TcpListener::bind(&config.get_server_url())
        .await
        .unwrap();

    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    api::axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db_url: String) -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting with shuttle");

    let conn = api::setup_db(&db_url).await;
    migrate(&conn).await.expect("Migration failed!");

    let router = api::setup_router(conn);
    Ok(router.into())
}
