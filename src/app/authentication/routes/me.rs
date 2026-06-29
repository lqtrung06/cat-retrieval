use actix_web::{HttpRequest, HttpResponse, web};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::SESSION_COOKIE_NAME;

#[derive(Serialize)]
struct MeResponse {
    username: String,
    email: Option<String>,
    role: String,
}

pub async fn me(db_pool: web::Data<PgPool>, request: HttpRequest) -> HttpResponse {
    let session_id = match request
        .cookie(SESSION_COOKIE_NAME)
        .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
    {
        Some(session_id) => session_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let user = sqlx::query_as!(
        MeResponse,
        r#"
        SELECT users.username, users.email, users.role
        FROM sessions
        JOIN users ON users.id = sessions.user_id
        WHERE sessions.id = $1
          AND sessions.expires_at > now()
          AND users.status = 'active'
        "#,
        session_id,
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
