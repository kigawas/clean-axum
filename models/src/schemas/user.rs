use serde::Serialize;

use crate::domains::user;

#[derive(Serialize)]
pub struct UserSchema {
    pub id: u32,
    pub username: String,
}

impl From<user::Model> for UserSchema {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id as u32,
            username: user.username,
        }
    }
}

#[derive(Serialize)]
pub struct UserListSchema {
    pub users: Vec<UserSchema>,
}

impl From<Vec<user::Model>> for UserListSchema {
    fn from(users: Vec<user::Model>) -> Self {
        Self {
            users: users.into_iter().map(UserSchema::from).collect(),
        }
    }
}
