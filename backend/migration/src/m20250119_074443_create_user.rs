use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(string(User::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            INSERT INTO "user" (id, name)
            VALUES ('test_admin_user', 'test_admin_user_name');
            INSERT INTO "user" (id, name)
            VALUES ('test_operator_user', 'test_operator_user_name');
            INSERT INTO "user" (id, name)
            VALUES ('test_customer_user', 'test_customer_user_name');
            "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum User {
    Table,
    Id,
    Name,
}
