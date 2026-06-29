use actix_web::HttpResponse;
use actix_web::web;

use super::authentication::routes::{login, me, register};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/health_check", web::get().to(health_check))
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/me", web::get().to(me));
}
