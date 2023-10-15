use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;

    let exp = now.timestamp() as usize;

    let claim = Claims { exp, iat };
    let secret: &'static str = dotenv!("JWT_SECRET");

    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");

    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(
        token,
        &key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
            StatusCode::UNAUTHORIZED,
            "session has expired, please login",
        ),
        jsonwebtoken::errors::ErrorKind::InvalidToken => {
            AppError::new(StatusCode::UNAUTHORIZED, "Invalid token")
        }
        jsonwebtoken::errors::ErrorKind::InvalidSignature => {
            AppError::new(StatusCode::UNAUTHORIZED, "Invalid token")
        }
        error => {
            println!("{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        }
    })?;
    Ok(true)
}
