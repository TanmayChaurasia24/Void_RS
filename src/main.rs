pub mod route;
pub mod input;
pub mod output;
pub mod orderbook;
pub mod database;


use actix_web::{App, HttpServer};
use crate::database::database_connection::connect_to_db;
use crate::{route::{create_order, delete_order, get_depth}};
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
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
