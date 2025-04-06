use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Default, FromRow, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(serialize_with = "crate::utils::date_format::serialize")]
    pub created: NaiveDateTime,
    #[serde(serialize_with = "crate::utils::date_format::serialize")]
    pub updated: NaiveDateTime,
}
