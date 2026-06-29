use crate::modules::authorization::dto::Role;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserInfo {
    pub email: Option<String>,
    pub username: String,
    pub role: Role,
}
