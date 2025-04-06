use chrono::Utc;

use actix_web::{HttpResponse, Responder};

#[utoipa::path(
    get,
    path ="/health",
    tag="Health",
    responses(
        (status = 200, description = "Check System uptime status"),

    )
)]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Service  is up and running as at {} utc!",
        Utc::now().format("%d-%m-%Y %H:%M:%S")
    ))
}
