use migration::{sea_orm::DatabaseConnection, DbErr, Migrator, MigratorTrait, SchemaManager};

pub async fn migrate(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(conn);
    Migrator::up(conn, None).await?;
    assert!(schema_manager.has_table("user").await?);
    Ok(())
}
