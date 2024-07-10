use crate::migrate;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn setup_test_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url).await?;
    migrate(&db).await?;
    Ok(db)
}
