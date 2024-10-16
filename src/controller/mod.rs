mod dto;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::config::DbPool;
use crate::controller::dto::{RegisterDto, LoginDto};
use crate::service::{register_user, login_user};
use crate::service::auth::{is_authenticated_admin};

#[post("/register")]
async fn register_user_controller(
    pool: web::Data<DbPool>,
    form: web::Json<RegisterDto>,
    req: HttpRequest  // Incluímos o HttpRequest para verificar o cabeçalho de autorização
) -> impl Responder {
    // Verifica se o usuário está autenticado e tem o papel de administrador
    match is_authenticated_admin(&req).await {
        Ok(_) => {
            let mut conn = pool.get().expect("Couldn't get DB connection");  // Obtenha a conexão mutável

            // Tenta registrar o usuário
            match register_user(&mut conn, &form.email, &form.password, &form.role) {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Admin access required"),  // Se não for admin
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
