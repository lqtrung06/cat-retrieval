#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    UserFree,
    UserPro,
    Admin,
}
