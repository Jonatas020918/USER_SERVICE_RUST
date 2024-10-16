use actix_web::{App, HttpServer, web};
mod config;
mod controller;
mod service;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = config::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(controller::register_user_controller)
            .service(controller::login_user_controller)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
