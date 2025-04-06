use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_env: String,
    pub redis_cache_url: String,
    pub database_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // Load .env in local dev

        Self {
            app_env: env::var("APP_ENV").expect("APP ENVIRONMENT  must be set"),
            redis_cache_url: env::var("REDIS_URL").expect("REDIS URL   must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
        }
    }
}
