use utils::testing::setup_test_db;

mod user;
use user::test_user;

#[tokio::test]
async fn main() {
    let db = setup_test_db("sqlite::memory:")
        .await
        .expect("Set up db failed!");

    test_user(&db).await;
}
