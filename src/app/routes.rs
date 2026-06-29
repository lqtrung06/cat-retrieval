use actix_web::HttpResponse;
use actix_web::web;

use crate::modules::authentication::routes::{login, me};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/health_check", web::get().to(health_check))
        .route("/login", web::post().to(login))
        .route("/me", web::get().to(me));
}
