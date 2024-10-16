use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
