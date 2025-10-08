use sea_orm::{DatabaseConnection, Database};
use std::env;
use dotenvy::dotenv;

pub async fn connect_to_db() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = Database::connect(&database_url).await.expect("Failed to connect to database");
    println!("Connected to database");
    database
}
