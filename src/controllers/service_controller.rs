use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub async fn status() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");

    let response = json!({
        "data": {
            "version": version,
        },
        "message": "Service is running..."
    });
    (StatusCode::OK, Json(response))
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/service/status", get(status))
}