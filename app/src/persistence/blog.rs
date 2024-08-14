use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

use models::{domains::blog, params::blog::CreateBlogParams, queries::blog::BlogQuery};

pub async fn search_blogs(db: &DbConn, query: BlogQuery) -> Result<Vec<blog::Model>, DbErr> {
    blog::Entity::find()
        .filter(blog::Column::Title.contains(query.title))
        .all(db)
        .await
}

pub async fn create_blog(
    db: &DbConn,
    params: CreateBlogParams,
) -> Result<blog::ActiveModel, DbErr> {
    blog::ActiveModel {
        author_id: Set(params.author_id as i32),
        title: Set(params.title),
        content: Set(params.content),
        ..Default::default()
    }
    .save(db)
    .await
}
