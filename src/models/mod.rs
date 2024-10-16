use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String, // Certifique-se de que este campo seja do tipo `String`
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub hashed_password: &'a str,
    pub role: &'a str,  // This should expect a reference to a string slice (&str)
    pub created_at: NaiveDateTime,
}