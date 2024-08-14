use axum::{http::StatusCode, Router};
use http_body_util::BodyExt;
use serde_json::Value;

use utils::testing::{make_get_request, make_post_request};

pub(super) async fn test_post_blogs(app: Router) {
    let response = make_post_request(
        app,
        "/blogs",
        r#"{"author_id": 1, "title": "title", "content": "test"}"#.to_owned(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);
}

pub(super) async fn test_get_blogs(app: Router) {
    let response = make_get_request(app, "/blogs").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let result: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(result["blogs"].as_array().unwrap().len(), 1);

    let blog = &result["blogs"][0];
    assert_eq!(blog["author_id"], 1);
    assert_eq!(blog["title"], "title");
    assert_eq!(blog["content"], "test");
}
