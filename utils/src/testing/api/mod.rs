use axum::{body::Body, http::Request, response::Response, Router};
use tower::ServiceExt;

pub async fn make_get_request(app: Router, url: &str) -> Response {
    app.oneshot(Request::builder().uri(url).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

pub async fn make_post_request(app: Router, url: &str, body: String) -> Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(url)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap(),
    )
    .await
    .unwrap()
}
