use hound::app;
use hound::infrastructure::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = Database.connect("production").await.map_err(|error| {
        std::io::Error::other(format!("Failed to connect to database: {error}"))
    })?;
    let port = app::run(Some(3000), Some(db_pool))?;
    println!("Server listening on port {port}");

    tokio::signal::ctrl_c().await
}
