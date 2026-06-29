use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: String,
    pub status: String,
}
