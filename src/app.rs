mod authentication;

use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

use authentication::routes::{login, me, register};
use sqlx::PgPool;

pub fn run(port: Option<u16>, db_pool: Option<PgPool>) -> Result<u16, std::io::Error> {
    let port = port.unwrap_or(0);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    let port = listener.local_addr()?.port();

    let server = HttpServer::new(move || {
        let app = App::new().configure(configure_routes);

        match db_pool.clone() {
            Some(db_pool) => app.app_data(web::Data::new(db_pool)),
            None => app,
        }
    })
    .listen(listener)?
    .run();

    tokio::spawn(server);

    Ok(port)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn configure_routes(config: &mut web::ServiceConfig) {
    config
        .route("/health_check", web::get().to(health_check))
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/me", web::get().to(me));
}
