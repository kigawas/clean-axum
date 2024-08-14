use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_init::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blog::Table)
                    .if_not_exists()
                    .col(pk_auto(Blog::Id))
                    .col(integer(Blog::AuthorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_blog_author_id")
                            .from(Blog::Table, Blog::AuthorId)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Blog::Title).string().not_null())
                    .col(ColumnDef::new(Blog::Content).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blog {
    Table,
    Id,
    AuthorId,
    Title,
    Content,
}
