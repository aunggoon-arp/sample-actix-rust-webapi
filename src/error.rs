use actix_web::{http::StatusCode, web::Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Password doesn't match")]
    WrongPassword,
    #[error("Email is already taken")]
    DuplicateUserEmail,
    #[error("Name is already taken")]
    DuplicateUserName,
    #[error("Search data not found")]
    NotFoundData,
}
pub type Result<T> = std::result::Result<T, Error>;
pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
