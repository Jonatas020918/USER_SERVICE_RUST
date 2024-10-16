use std::convert::AsRef;
use std::env;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use lazy_static::lazy_static;

lazy_static! {
    static ref SECRET_KEY: Vec<u8> = env::var("JWT_SECRET_TOKEN")
        .expect("JWT_SECRET_TOKEN must be set")
        .into_bytes();  // Converte a string para bytes
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // This could be the user's email or ID
    role: String, // Role (e.g., admin or user)
    exp: usize,   // Expiration time
}

pub(crate) fn create_jwt_token(user_id: &str, role: &str) -> String {
    let expiration = Utc::now() + Duration::hours(480);

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(&SECRET_KEY)).unwrap()
}

pub fn validate_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(&SECRET_KEY), &Validation::default())
        .map(|data| data.claims)
}


pub async fn is_authenticated_admin(req: &HttpRequest) -> Result<(), Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Extract token after "Bearer "
                match validate_jwt_token(token) {
                    Ok(claims) => {
                        if claims.role == "admin" {
                            return Ok(()); // User is an admin
                        }
                    }
                    Err(_) => return Err(ErrorUnauthorized("Invalid JWT")),
                }
            }
        }
    }
    Err(ErrorUnauthorized("Authorization header missing or invalid"))
}