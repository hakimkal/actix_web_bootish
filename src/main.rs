mod api_swagger_config;
mod config;
mod database;
mod handlers;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpResponse, HttpServer, web};

use crate::api_swagger_config::ApiConfig;
use crate::utils::redis_service_util::RedisService;
use config::AppConfig;
use database::connect_db;
use routes::register_routes;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa::openapi::{Info, Tag};
use utoipa_swagger_ui::SwaggerUi;

fn init_logger(app_env: &str) {
    let filter = if app_env != "prod" { "debug" } else { "info" };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();
    init_logger(&config.app_env);
    RedisService::init().await;
    let db_pool = connect_db(&config.database_url).await;
    let port = config.server_port;

    info!("Starting server on port {}", port);

    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(|cfg| register_routes(cfg));

        // ✅ Only enable Swagger if APP_ENV is not "prod"
        if config.app_env != "prod" {
            app = app.service(
                SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiConfig::openapi()),
            );
        }

        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
