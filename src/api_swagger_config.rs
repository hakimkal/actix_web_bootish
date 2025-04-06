use crate::handlers::home_handler::__path_health;
use crate::models::user::User;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
    paths(health),
    tags(
        (name = "Actix-Bootish", description = " API  endpoints")
    ),
    components(schemas(User)),

    info(description = "API Services Swagger docs",
        title=" API Management",
        contact(name="Abdulhakim Haliru",email="abdulhakim@paydestal.com"))
)]
pub struct ApiConfig;
