#[derive(Debug)]
pub enum UserError {
    NotFound,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for UserError {}
