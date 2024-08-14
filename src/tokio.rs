use api::{setup_config, setup_db, setup_router};
use doc::ApiDoc;
use utils::{create_dev_db, migrate};

async fn worker(child_num: u32, db_url: &str, prefork: bool, listener: std::net::TcpListener) {
    tracing::info!("Worker {} started", child_num);

    let conn = setup_db(db_url, prefork).await;

    if child_num == 0 {
        migrate(&conn).await.expect("Migration failed!");
    }

    let router = setup_router(conn).attach_doc();

    let listener = tokio::net::TcpListener::from_std(listener).expect("bind to port");
    axum::serve(listener, router).await.expect("start server");
}

#[cfg(feature = "prefork")]
fn run_prefork(db_url: &str, listener: std::net::TcpListener) {
    let db_url: &'static str = Box::leak(db_url.to_owned().into_boxed_str());
    create_dev_db(db_url);

    let num_of_cores = std::thread::available_parallelism().unwrap().get() as u32;
    let is_parent = prefork::Prefork::from_resource(listener)
        .with_num_processes(num_of_cores)
        .with_tokio(move |child_num, listener| worker(child_num, db_url, true, listener))
        .fork()
        .expect("prefork failed");

    if is_parent {
        tracing::info!("All workers stopped");
    }
}

fn run_non_prefork(db_url: &str, listener: std::net::TcpListener) {
    create_dev_db(db_url);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(worker(0, db_url, false, listener));
}

pub fn run() {
    let config = setup_config();

    let listener = std::net::TcpListener::bind(config.get_server_url()).expect("bind to port");
    listener.set_nonblocking(true).expect("non blocking failed");
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

    #[cfg(feature = "prefork")]
    if config.prefork {
        run_prefork(&config.db_url, listener);
        return;
    }

    run_non_prefork(&config.db_url, listener);
}
