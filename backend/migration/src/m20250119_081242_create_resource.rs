use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resource::Table)
                    .if_not_exists()
                    .col(pk_auto(Resource::Id))
                    .col(string(Resource::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
        INSERT INTO "resource" (id, name)
        VALUES (1, 'Product');
        INSERT INTO "resource" (id, name)
        VALUES (2, 'Order');
        INSERT INTO "resource" (id, name)
        VALUES (3, 'Customer');
        INSERT INTO "resource" (id, name)
        VALUES (4, 'Inventory');
        INSERT INTO "resource" (id, name)
        VALUES (5, 'DraftOrder');
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resource::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Resource {
    Table,
    Id,
    Name,
}
