use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Priority,
    Title,
    CompletedAt,
    Description,
    DeletedAt,
    UserId,
    IsDefault,
}

// CREATE TABLE IF NOT EXISTS tasks (
//   id            SERIAL PRIMARY KEY,
//   priority      VARCHAR(4) DEFAULT NULL,
//   title         VARCHAR(255) NOT NULL,
//   completed_at  TIMESTAMPTZ DEFAULT NULL,
//   description   TEXT DEFAULT NULL,
//   deleted_at    TIMESTAMPTZ DEFAULT NULL,
//   user_id       INTEGER DEFAULT NULL,
//   is_default    BOOLEAN DEFAULT FALSE,
//   CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
// );

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Task::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Task::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Task::Priority).string().string_len(4).null())
            .col(
                ColumnDef::new(Task::Title)
                    .string()
                    .string_len(255)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Task::CompletedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .col(ColumnDef::new(Task::Description).text().null())
            .col(
                ColumnDef::new(Task::DeletedAt)
                    .timestamp_with_time_zone()
                    .null(),
            )
            .col(ColumnDef::new(Task::UserId).integer().null())
            .col(ColumnDef::new(Task::IsDefault).boolean().default(false))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}
