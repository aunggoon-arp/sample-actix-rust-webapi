use sqlx::PgPool;

use crate::dto::user::{RegisterInput, CreateUserData, UpdateUserInput, UpdateUserData};
use crate::entity::user::User;
use crate::error::{Error, Result};
use crate::utils::encryption;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
        let user = User::find_user_by_id(id, &pool).await?;
        Ok(user)
    }

    pub async fn create_user(input: RegisterInput, pool: &PgPool) -> Result<User> {
        if User::find_user_by_email(&input.email, &pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }
        if User::find_user_by_name(&input.name, &pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }
        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
        };
        Ok(User::create_user(data, &pool).await?)
    }

    pub async fn update_user(input: UpdateUserInput, pool: &PgPool) -> Result<User> {
        if User::find_user_by_email(&input.email, &pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }
        if User::find_user_by_name(&input.name, &pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }
        let data = UpdateUserData {
            name: input.name,
            email: input.email,
        };
        Ok(User::update_user(data, &pool).await?)
    }
}
