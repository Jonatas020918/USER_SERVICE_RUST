use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use bcrypt::{hash, verify};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;
use diesel::result::Error;

pub fn register_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,  // Conexão mutável
    user_email: &str,  // Rename the parameter to `user_email`
    password: &str,
    user_role: &str
) -> Result<User, Error> {
    // Hash the password using bcrypt and get it as a String
    let user_hashed_password = hash(password, 4).expect("Failed to hash password");

    // Create a new user
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: user_email,  // Use `user_email` instead of `email` to avoid ambiguity
        hashed_password: &user_hashed_password,
        role: &user_role,
        created_at: Utc::now().naive_utc(),
    };

    // Insert the new user into the database
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)?;

    // Fetch the newly created user
    let created_user = users
        .filter(email.eq(user_email))  // Use `user_email` here as well
        .first::<User>(conn)?;

    Ok(created_user)
}

pub fn login_user(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, user_email: &str, password: &str) -> Result<User, Error> {
    // O uso de `&mut` garante que a conexão será mutável
    let user = users
        .filter(email.eq(user_email))
        .first::<User>(conn)?; // Passar `conn` como referência mutável

    // Verifique a senha fornecida com a senha armazenada (hashed_password)
    if verify(password, &user.hashed_password).unwrap() {
        Ok(user)
    } else {
        Err(Error::NotFound)
    }
}
