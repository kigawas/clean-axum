use axum::{http::StatusCode, Router};
use http_body_util::BodyExt;
use serde_json::Value;

use utils::testing::{make_get_request, make_post_request};

pub(super) async fn test_post_users(app: Router) {
    let response = make_post_request(app, "/users", r#"{"username": "test"}"#.to_owned()).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], br#"{"id":1,"username":"test"}"#);
}

pub(super) async fn test_post_users_error(app: Router) {
    let response = make_post_request(app, "/users", r#"{"username": "1"}"#.to_owned()).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let result: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(result["message"], "Validation error");
    assert_eq!(result["details"]["username"][0]["code"], "length");
    assert_eq!(result["details"]["username"][0]["message"], Value::Null);
    assert_eq!(
        result["details"]["username"][0]["params"]["min"],
        Value::Number(2.into())
    )
}

pub(super) async fn test_get_users(app: Router) {
    let response = make_get_request(app, "/users").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], br#"{"users":[{"id":1,"username":"test"}]}"#);
}
