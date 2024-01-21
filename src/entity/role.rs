use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Role {
    #[serde(skip_serializing)]
    pub id: i32,
    pub name_th: String,
    pub name_en: String,
    pub role_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub const TABLE: &'static str = "role_info";
}
