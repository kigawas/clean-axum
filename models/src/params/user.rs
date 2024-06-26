use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}
