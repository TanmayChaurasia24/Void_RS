pub mod auth;

use actix_web::web;

/// Register all routes in this function
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    auth::init_routes(cfg);
}
