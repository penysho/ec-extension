use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            INSERT INTO "role" (id, name)
            VALUES (1, 'admin');
            INSERT INTO "role" (id, name)
            VALUES (2, 'operator');
            INSERT INTO "role" (id, name)
            VALUES (3, 'customer');
            INSERT INTO "role" (id, name)
            VALUES (4, 'not_login');
            "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Role {
    Table,
    Id,
    Name,
}
