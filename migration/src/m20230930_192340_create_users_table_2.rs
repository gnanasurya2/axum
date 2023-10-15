use std::ptr::null;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
    #[sea_orm(default_expr = "NULL")]
    DeletedAt,
    Token,
}

// CREATE TABLE IF NOT EXISTS users (
//   id          SERIAL PRIMARY KEY,
//   username    VARCHAR(64) NOT NULL UNIQUE,
//   password    VARCHAR(64) NOT NULL,
//   deleted_at  TIMESTAMPTZ DEFAULT NULL,
//   token       TEXT DEFAULT NULL
// );

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Users::Username)
                    .string()
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Users::Password)
                    .string()
                    .string_len(64)
                    .not_null(),
            )
            .col(ColumnDef::new(Users::DeletedAt).timestamp().null())
            .col(ColumnDef::new(Users::Token).string().null())
            .to_owned();
        println!("create table {:?}", table.to_string(MysqlQueryBuilder));
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
