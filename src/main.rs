use std::env;
use actix_web::{App, HttpServer, web};
mod config;
mod controller;
mod service;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = config::init_pool();
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set").parse().expect("PORT must be able to parse to a u16");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(controller::register_user_controller)
            .service(controller::login_user_controller)
    })
        .bind((host, port))?
        .run()
        .await
}
