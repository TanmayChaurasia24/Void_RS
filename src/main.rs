use actix_web::{App, HttpServer, Responder, post};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || App::new().service(create_order))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[post("/order")]
async fn create_order() -> impl Responder {
    "this is create order route handler"
}
