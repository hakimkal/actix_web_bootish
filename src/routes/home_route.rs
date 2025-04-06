use crate::handlers::home_handler;
use actix_web::web;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(home_handler::health));
}
