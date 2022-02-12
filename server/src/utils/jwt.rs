use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: usize,          // Expiration timestamp
    pub iat: usize,          // Issued At timestamp
    pub sub: String,         // Subject (user ID)
}

pub trait IntoJwt {
    fn into_jwt(self, current_timestamp: DateTime<Utc>, duration: Duration, algorithm: Algorithm, key: EncodingKey) -> jsonwebtoken::errors::Result<String>;
}

pub struct JwtConfig {
    pub algorithm: Algorithm,
    pub secret: String
}
