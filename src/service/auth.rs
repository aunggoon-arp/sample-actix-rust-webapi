use sqlx::PgPool;

use crate::dto::user::LoginInput;
use crate::entity::user::User;
use crate::{
    error::{Error, Result},
    utils::encryption,
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(input: LoginInput, pool: &PgPool) -> Result<User> {
        let user = User::find_user_by_email(&input.email, &pool).await?;
        if user.is_some() && encryption::verify_password(input.password, user.clone().unwrap().password.to_owned()).await? {
            Ok(user.unwrap())
        } else {
            Err(Error::WrongPassword)
        }
    }
}
