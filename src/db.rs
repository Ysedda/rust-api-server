use dotenv::{dotenv, var};
use sqlx::postgres::PgPoolOptions;

pub async fn connect_to_db() {
    dotenv().ok();
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgPoolOptions::new()
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
}
