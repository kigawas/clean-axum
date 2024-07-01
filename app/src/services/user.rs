use models::domains::user;
use models::orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};
use models::params::user::CreateUserParams;
use models::queries::user::UserQuery;

pub async fn create_user(
    db: &DbConn,
    params: CreateUserParams,
) -> Result<user::ActiveModel, DbErr> {
    user::ActiveModel {
        username: Set(params.username),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn search_users(db: &DbConn, query: UserQuery) -> Result<Vec<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Username.contains(query.username))
        .all(db)
        .await
}

pub async fn get_user(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}
