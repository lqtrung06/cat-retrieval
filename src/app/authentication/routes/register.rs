use actix_web::{HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::user::dto::UserRow;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    confirm_password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    username: String,
    email: Option<String>,
    role: String,
}

pub async fn register(
    db_pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    if body.password != body.confirm_password {
        return HttpResponse::BadRequest().finish();
    }

    let password_hash = match hash(&body.password, DEFAULT_COST) {
        Ok(password_hash) => password_hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let user_id = Uuid::new_v4();
    let user = sqlx::query_as!(
        UserRow,
        r#"
        INSERT INTO users (id, username, password_hash, role, status)
        VALUES ($1, $2, $3, 'user_free', 'active')
        RETURNING id, username, email, password_hash, role, status
        "#,
        user_id,
        body.username,
        password_hash,
    )
    .fetch_one(db_pool.get_ref())
    .await;

    const UNIQUE_VIOLATION_ERROR_CODE: &str = "23505";
    match user {
        Ok(user) => HttpResponse::Created().json(RegisterResponse {
            username: user.username,
            email: user.email,
            role: user.role,
        }),
        Err(sqlx::Error::Database(error))
            if error.code().as_deref() == Some(UNIQUE_VIOLATION_ERROR_CODE) =>
        {
            HttpResponse::Conflict().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
