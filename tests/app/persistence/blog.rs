use sea_orm::{DatabaseConnection, Unchanged};

use app::persistence::blog::create_blog;
use models::domains::blog;
use models::params::blog::CreateBlogParams;

pub(super) async fn test_blog(db: &DatabaseConnection) {
    let params = CreateBlogParams {
        author_id: 1,
        title: "title".to_string(),
        content: "test".to_string(),
    };

    let blog = create_blog(db, params).await.expect("Create blog failed!");
    let expected = blog::ActiveModel {
        id: Unchanged(1),
        author_id: Unchanged(1),
        title: Unchanged("title".to_owned()),
        content: Unchanged("test".to_owned()),
    };
    assert_eq!(blog, expected);
}
