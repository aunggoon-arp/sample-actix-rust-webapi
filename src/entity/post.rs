use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Post {
    pub id: i32,
    pub type_id: i32,
    pub description: String,
    pub created_user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

impl Post {
    pub const TABLE: &'static str = "post_info";
}
