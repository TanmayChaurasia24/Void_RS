use actix_web::{delete, get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


//------------------------------------ Create Order ------------------------------------
#[derive(Deserialize, Debug)]
struct CreateOrderInput {
    price: u32,
    quantity: u32,
    user_id: u32,
    order_type: OrderType
}

#[derive(Deserialize, Debug)]
enum OrderType {
    Buy,
    Sell
}

#[derive(serde::Serialize, Debug)]
struct CreateOrderResponse {
    order_id: String,
    status: String
}

#[post("/order")]
async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    println!("{:?}", body);

    let _price = body.0.price;
    let _quantity = body.0.quantity;
    let _user_id = body.0.user_id;
    let _order_type = body.0.order_type;

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads"),
        status: "Order created successfully".to_string()
    })
}


//------------------------------------ Delete Order ------------------------------------
#[delete("order")]
async fn delete_order() -> impl Responder {
    "this is the delete order route handler"
}

//------------------------------------ Get Depth ------------------------------------
#[get("/depth")]
async fn get_depth() -> impl Responder {
    "this is the get depth route handler"
}

