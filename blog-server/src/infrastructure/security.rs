use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TOKEN_LIFETIME_HOURS: i64 = 24;
const TOKEN_EXPIRATION_LEEWAY: u64 = 60;

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    pub fn generate_token(
        &self,
        user_id: Uuid,
        username: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: Utc::now()
                .checked_add_signed(Duration::hours(TOKEN_LIFETIME_HOURS))
                .unwrap()
                .timestamp() as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = TOKEN_EXPIRATION_LEEWAY;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;
        Ok(data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
}
