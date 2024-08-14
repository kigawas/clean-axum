use utils::testing::setup_test_db;

mod blog;
mod user;

use blog::test_blog;
use user::test_user;

#[tokio::test]
async fn user_main() {
    let db = setup_test_db("sqlite::memory:")
        .await
        .expect("Set up db failed!");

    test_user(&db).await;
}

#[tokio::test]
async fn blog_main() {
    let db = setup_test_db("sqlite::memory:")
        .await
        .expect("Set up db failed!");

    test_user(&db).await;
    test_blog(&db).await;
}
