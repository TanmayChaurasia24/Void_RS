pub mod routes;
pub mod database;
pub mod lib;
pub mod services;


use actix_web::{App, HttpServer};
use crate::database::database_connection::connect_to_db;
use std::sync::{Arc, Mutex};
use std::env;
use dotenvy::dotenv;



#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    println!("DATABASE_URL={}", env::var("DATABASE_URL").unwrap());
    let db = connect_to_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone())) // wrap in Data
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
