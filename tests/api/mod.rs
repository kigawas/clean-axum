use api::setup_router;
use utils::testing::setup_test_db;

mod blog;
mod root;
mod user;

use blog::*;
use root::*;
use user::*;

#[tokio::test]
async fn root_main() {
    let db = setup_test_db("sqlite::root?mode=memory&cache=shared")
        .await
        .expect("Set up db failed!");

    let app = setup_router(db);
    test_root(app).await;
}

#[tokio::test]
async fn user_main() {
    let db = setup_test_db("sqlite::user?mode=memory&cache=shared")
        .await
        .expect("Set up db failed!");

    let app = setup_router(db);
    test_post_users(app.clone()).await;
    test_post_users_error(app.clone()).await;
    test_get_users(app).await;
}

#[tokio::test]
async fn blog_main() {
    let db = setup_test_db("sqlite::blog?mode=memory&cache=shared")
        .await
        .expect("Set up db failed!");

    let app = setup_router(db);
    test_post_users(app.clone()).await;
    test_post_blogs(app.clone()).await;
    test_get_blogs(app).await;
}
