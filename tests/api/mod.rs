use http_body_util::BodyExt;

use api::axum::{http::StatusCode, Router};
use api::setup_router;
use utils::testing::{make_get_request, make_post_request, setup_test_db};

#[tokio::test]
async fn main() {
    let db = setup_test_db("sqlite::memory:")
        .await
        .expect("Set up db failed!");

    let app = setup_router(db);
    test_root(app.clone()).await;
    test_post_users(app.clone()).await;
    test_get_users(app).await;
}

async fn test_root(app: Router) {
    let response = make_get_request(app, "/").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World from DB!");
}

async fn test_post_users(app: Router) {
    let response = make_post_request(app, "/users", r#"{"username": "test"}"#.to_owned()).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], br#"{"id":1,"username":"test"}"#);
}

async fn test_get_users(app: Router) {
    let response = make_get_request(app, "/users").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], br#"{"users":[{"id":1,"username":"test"}]}"#);
}
