use chrono::Local;
use sqlx::PgPool;

use crate::{
    error::Result,
    dto::user::{CreateUserData, UpdateUserData},
    entity::user::User
};

impl User {
    pub async fn find_user_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
        let sql = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_optional(pool).await?)
    }

    pub async fn find_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>> {
        let sql = format!("SELECT * FROM {} WHERE email = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(email).fetch_optional(pool).await?)
    }

    pub async fn find_user_by_name(name: &str, pool: &PgPool) -> Result<Option<User>> {
        let sql = format!("SELECT * FROM {} WHERE name = $1 LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(name).fetch_optional(pool).await?)
    }

    pub async fn create_user(data: CreateUserData, pool: &PgPool) -> Result<User> {
        let sql = format!(
            "
            INSERT INTO {} (name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
            User::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.name)
            .bind(data.email)
            .bind(data.password)
            .bind(Local::now())
            .bind(Local::now())
            .fetch_one(pool)
            .await?)
    }

    pub async fn update_user(data: UpdateUserData, pool: &PgPool) -> Result<User> {
        let sql = format!(
            "
            UPDATE {}
            SET name = $1, email = $2, updated_at = $3)
            RETURNING *
            ",
            User::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.name)
            .bind(data.email)
            .bind(Local::now())
            .fetch_one(pool)
            .await?)
    }
}
