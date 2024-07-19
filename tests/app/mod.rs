use sea_orm::{DatabaseConnection, DbErr, Unchanged};

use app::services::user::create_user;
use models::domains::user;
use models::params::user::CreateUserParams;
use utils::testing::setup_test_db;

#[tokio::test]
async fn main() -> Result<(), DbErr> {
    let db = setup_test_db("sqlite::memory:").await?;
    test_user(&db).await?;
    Ok(())
}

async fn test_user(db: &DatabaseConnection) -> Result<(), DbErr> {
    let params = CreateUserParams {
        username: "test".to_string(),
    };

    let user = create_user(db, params).await?;
    let expected = user::ActiveModel {
        id: Unchanged(1),
        username: Unchanged("test".to_owned()),
    };
    assert_eq!(user, expected);
    Ok(())
}
