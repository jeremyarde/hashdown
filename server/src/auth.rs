use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::ServerError;

pub enum AuthError {
    PasswordDoNotMatch,
}

pub async fn validate_credentials(
    expected_password_hash: String,
    password_candidate: String,
) -> anyhow::Result<(), ServerError> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash).unwrap();
    match Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    {
        Ok(x) => return Ok(()),
        Err(e) => return Err(ServerError::PasswordDoesNotMatch),
    };
}
