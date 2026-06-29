use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Copy)]
pub struct DatabaseConfig {
    host: &'static str,
    port: u16,
    username: &'static str,
    password: &'static str,
    database_name: &'static str,
}

pub struct DevelopmentDatabase;

pub struct ProductionDatabase;

trait Connection {
    fn has_credential(&self) -> DatabaseConfig;

    fn connection_string(&self) -> String {
        let info = self.has_credential();

        format!(
            "postgres://{}:{}@{}:{}/{}",
            info.username, info.password, info.host, info.port, info.database_name
        )
    }
}

impl DevelopmentDatabase {
    pub async fn connect(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.connection_string())
            .await
    }
}

impl ProductionDatabase {
    pub async fn connect(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.connection_string())
            .await
    }
}

impl Connection for DevelopmentDatabase {
    fn has_credential(&self) -> DatabaseConfig {
        DatabaseConfig {
            host: "127.0.0.1",
            port: 5432,
            username: "postgres",
            password: "password",
            database_name: "development",
        }
    }
}

impl Connection for ProductionDatabase {
    fn has_credential(&self) -> DatabaseConfig {
        DatabaseConfig {
            host: "127.0.0.1",
            port: 5432,
            username: "postgres",
            password: "password",
            database_name: "production",
        }
    }
}
