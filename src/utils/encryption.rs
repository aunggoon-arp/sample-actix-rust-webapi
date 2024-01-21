use bcrypt::{DEFAULT_COST, BcryptResult};

pub async fn hash_password(password: String) -> BcryptResult<String> {
    bcrypt::hash(password, DEFAULT_COST)
}

pub async fn verify_password(password: String, hash: String) -> BcryptResult<bool> {
    bcrypt::verify(password, &hash)
}
