use std::sync::Arc;

use dotenv::{dotenv, var};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

mod db;
mod handlers;
mod models;
mod route;
mod schema;

use route::create_router;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    println!("Server started at http://localhost:8080");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
