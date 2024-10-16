mod dto;
use actix_web::{post, web, HttpResponse, Responder};
use crate::config::DbPool;
use crate::controller::dto::{RegisterDto, LoginDto};
use crate::service::{register_user, login_user};

#[post("/register")]
async fn register_user_controller(pool: web::Data<DbPool>, form: web::Json<RegisterDto>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");  // Obtenha a conexão mutável

    match register_user(&mut conn, &form.email, &form.password, &form.role) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
    }
}

#[post("/login")]
async fn login_user_controller(pool: web::Data<DbPool>, form: web::Json<LoginDto>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");  // Obtenha uma conexão mutável

    match login_user(&mut conn, &form.email, &form.password) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
