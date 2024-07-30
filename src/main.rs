#[cfg(not(feature = "shuttle"))]
mod tokio;

#[cfg(not(feature = "shuttle"))]
fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,clean_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting with tokio");
    tokio::run();
}

#[cfg(feature = "shuttle")]
mod shuttle;

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db_url: String) -> shuttle_axum::ShuttleAxum {
    shuttle::run(&db_url).await
}
