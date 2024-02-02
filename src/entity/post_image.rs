use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct PostImage {
    pub id: i32,
    pub filename: String,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

impl PostImage {
    pub const TABLE: &'static str = "post_image";
}
