use axum::{http::StatusCode, routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(task_1))
        .route("/-1/error", get(task_2))
}

async fn task_1() -> &'static str {
    "Hello, world!"
}

async fn task_2() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
