use axum::{response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthcheck", get(health_check_handler));

    println!("Server started at http://localhost:8080");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API";

    let json_response = serde_json::json!({
        "message": MESSAGE,
        "status": "OK"
    });

    return Json(json_response);
}
