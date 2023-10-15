pub use sea_orm_migration::prelude::*;

mod m20230930_174459_create_table;
mod m20230930_184520_create_users_table;
mod m20230930_192340_create_users_table_2;
mod m20230930_192834_create_tasks_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230930_174459_create_table::Migration),
            Box::new(m20230930_184520_create_users_table::Migration),
            Box::new(m20230930_192340_create_users_table_2::Migration),
            Box::new(m20230930_192834_create_tasks_table::Migration),
        ]
    }
}
