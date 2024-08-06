use sea_orm::{DatabaseConnection, Unchanged};

use app::persistence::user::create_user;
use models::domains::user;
use models::params::user::CreateUserParams;

pub(super) async fn test_user(db: &DatabaseConnection) {
    let params = CreateUserParams {
        username: "test".to_string(),
    };

    let user = create_user(db, params).await.expect("Create user failed!");
    let expected = user::ActiveModel {
        id: Unchanged(1),
        username: Unchanged("test".to_owned()),
    };
    assert_eq!(user, expected);
}
