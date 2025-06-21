use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(UserGroup::Id))
                    .col(string(UserGroup::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO "user_group" (id, name)
                VALUES (1, 'test_admin_user_group');
                INSERT INTO "user_group" (id, name)
                VALUES (2, 'test_operator_user_group');
                INSERT INTO "user_group" (id, name)
                VALUES (3, 'test_customer_user_group');
                INSERT INTO "user_group" (id, name)
                VALUES (4, 'test_not_login_user_group');
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum UserGroup {
    Table,
    Id,
    Name,
}
