use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod db;
mod handlers;

#[tokio::main]
async fn main() {
    db::connect_to_db().await;

    let app = Router::new()
        .route("/api/healthcheck", get(handlers::health_check_handler))
        .route("/", get(handlers::user_handler));

    println!("Server started at http://localhost:8080");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
