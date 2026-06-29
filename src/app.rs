mod authentication;
pub mod routes;

use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

use sqlx::PgPool;

pub fn run(port: Option<u16>, db_pool: PgPool) -> Result<u16, std::io::Error> {
    let port = port.unwrap_or(0);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    let port = listener.local_addr()?.port();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::configure)
    })
    .listen(listener)?
    .run();

    tokio::spawn(server);

    Ok(port)
}
