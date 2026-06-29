use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;

pub struct Database;

impl Database {
    const HOST: &'static str = "127.0.0.1";
    const PORT: u16 = 5432;
    const USERNAME: &'static str = "postgres";
    const PASSWORD: &'static str = "easy";

    pub async fn connect(&self, database_name: &str) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                Self::USERNAME,
                Self::PASSWORD,
                Self::HOST,
                Self::PORT,
                database_name
            ))
            .await
    }

    pub async fn create(&self, database_name: &str) -> Result<(), sqlx::Error> {
        let maintenance_pool = self.connect("postgres").await?;
        sqlx::query(&format!(
            "CREATE DATABASE {}",
            quote_database_name(database_name)
        ))
        .execute(&maintenance_pool)
        .await?;
        maintenance_pool.close().await;

        Ok(())
    }

    pub async fn clone(&self, database_name: &str) -> Result<PgPool, Box<dyn Error + Send + Sync>> {
        self.create(database_name).await?;

        let pool = self.connect(database_name).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(pool)
    }

    pub async fn drop(&self, database_name: &str) -> Result<(), sqlx::Error> {
        let maintenance_pool = self.connect("postgres").await?;

        sqlx::query(
            r#"
            SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = $1
              AND pid <> pg_backend_pid()
            "#,
        )
        .bind(database_name)
        .execute(&maintenance_pool)
        .await?;

        sqlx::query(&format!(
            "DROP DATABASE {}",
            quote_database_name(database_name)
        ))
        .execute(&maintenance_pool)
        .await?;
        maintenance_pool.close().await;

        Ok(())
    }
}

fn quote_database_name(database_name: &str) -> String {
    assert!(!database_name.is_empty(), "database name cannot be empty");

    format!(r#""{}""#, database_name.replace('"', r#""""#))
}
