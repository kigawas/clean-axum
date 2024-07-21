use axum::{http::StatusCode, Router};
use http_body_util::BodyExt;

use utils::testing::make_get_request;

pub(super) async fn test_root(app: Router) {
    let response = make_get_request(app, "/").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World from DB!");
}
