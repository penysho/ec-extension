use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string(Permission::Action))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
        INSERT INTO "permission" (id, action)
        VALUES (1, 'all');
        INSERT INTO "permission" (id, action)
        VALUES (2, 'own_read');
        INSERT INTO "permission" (id, action)
        VALUES (3, 'own_write');
        INSERT INTO "permission" (id, action)
        VALUES (4, 'own_delete');
        INSERT INTO "permission" (id, action)
        VALUES (5, 'all_read');
        INSERT INTO "permission" (id, action)
        VALUES (6, 'all_write');
        INSERT INTO "permission" (id, action)
        VALUES (7, 'all_delete');
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Permission {
    Table,
    Id,
    Action,
}
