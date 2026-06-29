use actix_web::cookie::{Cookie, SameSite, time::Duration};
use actix_web::{HttpRequest, HttpResponse, web};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

const SESSION_COOKIE_NAME: &str = "session_id";
const SESSION_MAX_AGE_SECONDS: i64 = 60 * 60 * 24;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    username: String,
    email: Option<String>,
    role: String,
    status: String,
}

struct StoredUser {
    id: Uuid,
    username: String,
    email: Option<String>,
    password_hash: String,
    role: String,
    status: String,
}

struct SessionUser {
    id: Uuid,
    username: String,
    email: Option<String>,
    role: String,
    status: String,
}

pub async fn login(db_pool: web::Data<PgPool>, body: web::Json<LoginRequest>) -> HttpResponse {
    let user = match find_user_by_username(&db_pool, &body.username).await {
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
    let insert_result = sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id, expires_at)
        VALUES ($1, $2, now() + ($3::text || ' seconds')::interval)
        "#,
    )
    .bind(session_id)
    .bind(user.id)
    .bind(SESSION_MAX_AGE_SECONDS)
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

    HttpResponse::Ok().cookie(cookie).json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
        status: user.status,
    })
}

pub async fn me(db_pool: web::Data<PgPool>, request: HttpRequest) -> HttpResponse {
    let session_id = match request
        .cookie(SESSION_COOKIE_NAME)
        .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
    {
        Some(session_id) => session_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match find_user_by_session_id(&db_pool, session_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            status: user.status,
        }),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn find_user_by_username(
    db_pool: &PgPool,
    username: &str,
) -> Result<Option<StoredUser>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, username, email, password_hash, role, status
        FROM users
        WHERE username = $1
          AND status = 'active'
        "#,
    )
    .bind(username)
    .fetch_optional(db_pool)
    .await?;

    Ok(row.map(|row| StoredUser {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        status: row.get("status"),
    }))
}

async fn find_user_by_session_id(
    db_pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<SessionUser>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT users.id, users.username, users.email, users.role, users.status
        FROM sessions
        JOIN users ON users.id = sessions.user_id
        WHERE sessions.id = $1
          AND sessions.expires_at > now()
          AND users.status = 'active'
        "#,
    )
    .bind(session_id)
    .fetch_optional(db_pool)
    .await?;

    Ok(row.map(|row| SessionUser {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
        role: row.get("role"),
        status: row.get("status"),
    }))
}
