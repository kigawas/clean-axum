use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct UserQuery {
    pub username: String,
}
