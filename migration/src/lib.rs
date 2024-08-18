pub use sea_orm_migration::prelude::*;

mod m20240101_000001_init;
mod m20240816_160144_blog;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_init::Migration),
            Box::new(m20240816_160144_blog::Migration),
        ]
    }
}
