use actix_web::cookie::{Cookie, SameSite, time::Duration};
use actix_web::{HttpResponse, web};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::user::dto::UserRow;

use super::SESSION_COOKIE_NAME;

const SESSION_MAX_AGE_SECONDS: i64 = 60 * 60 * 24;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    username: String,
    email: Option<String>,
    role: String,
}

pub async fn login(db_pool: web::Data<PgPool>, body: web::Json<LoginRequest>) -> HttpResponse {
    let user = match sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, username, email, password_hash, role, status
        FROM users
        WHERE username = $1
          AND status = 'active'
        "#,
        body.username,
    )
    .fetch_optional(db_pool.get_ref())
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match verify(&body.password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let session_id = Uuid::new_v4();
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, expires_at)
        VALUES ($1, $2, now() + ($3 * interval '1 second'))
        "#,
        session_id,
        user.id,
        SESSION_MAX_AGE_SECONDS as f64,
    )
    .execute(db_pool.get_ref())
    .await;

    if insert_result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let cookie = Cookie::build(SESSION_COOKIE_NAME, session_id.to_string())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(SESSION_MAX_AGE_SECONDS))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(LoginResponse {
        username: user.username,
        email: user.email,
        role: user.role,
    })
}
