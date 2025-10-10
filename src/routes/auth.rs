use actix_web::{web, HttpResponse, Responder};
use entity::prelude::*;
use crate::lib::auth::request_structure::RegisterRequestStructure;
use crate::lib::auth::response_structure::RegisterResponseStructure;
use crate::lib::auth::response_structure::LoginResponseStructure;
use crate::lib::auth::request_structure::LoginRequestStructure;

use crate::services::auth_services;

pub async fn register(
    db: web::Data<sea_orm::DatabaseConnection>,
    info: web::Json<RegisterRequestStructure>,
) -> impl Responder {
    println!("Received register request: {:?}", info); 

    if info.full_name.is_empty() || info.email.is_empty() || info.password.is_empty() {
        return HttpResponse::BadRequest().body("Missing required fields");
    }

    let existing_user = user::Entituy::find()
        .filter(user::Column::Email.eq(info.email.clone())).one(db.get_ref()).await.unwrap();

    if existing_user.is_some() {
        return HttpResponse::BadRequest().body("user already exists, try with some other email!");
    }

    println!("hashing password");
    let hashed_password = auth_services::hash_password(&info.password);
    println!("hashed password: {:?}", hashed_password);

    let user = user::ActiveModel {
        full_name: Set(info.full_name.clone()),
        email: Set(info.email.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };

    let user = user.insert(db.get_ref()).await.unwrap();

    if user.is_err() {
        return HttpResponse::InternalServerError().body("Error creating user");
    }

    return HttpResponse::Ok().json(RegisterResponseStructure {
        user_id: user.id.to_string(),
        message: "User registered successfully".to_string(),
    });
}

pub async fn login (
    db: web::Data<sea_orm::DatabaseConnection>,
    info: web::Json<RegisterRequestStructure>,
) -> impl Responder {
    println!("Received login request: {:?}", info); 

    if info.email.is_empty() || info.password.is_empty() {
        return HttpResponse::BadRequest().body("Missing required fields");
    }

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(info.email.clone()))
        .one(db.get_ref())
        .await
        .unwrap();

    if user.is_none() {
        return HttpResponse::BadRequest().body("Invalid email or password");
    }

    let user = user.unwrap();
    let hashed_password = user.password.clone();

    if !auth_services::verify_password(&info.password, &hashed_password) {
        return HttpResponse::BadRequest().body("Invalid email or password");
    }

    return HttpResponse::Ok().json(LoginResponseStructure {
        user_id: user.id.to_string(),
        token: auth_services::generate_jwt(user.id),
        message: "User logged in successfully".to_string(),
    });
}


/// Initialize auth routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}