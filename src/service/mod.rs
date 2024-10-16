use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use bcrypt::{hash, verify};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::result::Error;
use crate::service::auth::{create_jwt_token};

pub(crate) mod auth;

pub fn register_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,  // Conexão mutável
    user_email: &str,
    password: &str,
    user_role: &str
) -> Result<User, Error> {
    // Hash da senha usando bcrypt
    let user_hashed_password = hash(password, 4).expect("Falha ao hashear a senha");

    // Criar um novo usuário
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: user_email,  // Usar user_email em vez de email para evitar ambiguidade
        hashed_password: &user_hashed_password,
        role: &user_role,
        created_at: Utc::now().naive_utc(),
    };

    // Inserir o novo usuário no banco de dados
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)?;

    // Buscar o usuário recém-criado
    let created_user = users
        .filter(email.eq(user_email))
        .first::<User>(conn)?;

    Ok(created_user)
}

pub fn login_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_email: &str,
    password: &str
) -> Result<String, Error> {
    // Buscar o usuário pelo email
    let user = users
        .filter(email.eq(user_email))
        .first::<User>(conn)?;

    // Verificar se a senha está correta
    if verify(password, &user.hashed_password).unwrap() {
        // Se a senha estiver correta, gerar o token JWT
        let token = create_jwt_token(&user.id.to_string(), &user.role);
        Ok(token) // Retornar o token JWT
    } else {
        Err(Error::NotFound)
    }
}

