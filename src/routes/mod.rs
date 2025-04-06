pub mod home_route;

use actix_web::web;
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    home_route::configure(cfg);
}
