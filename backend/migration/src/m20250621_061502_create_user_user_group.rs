use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserUserGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(UserUserGroup::Id))
                    .col(string(UserUserGroup::UserId))
                    .col(string(UserUserGroup::UserGroupId))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_admin_user_2', 1);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_operator_user_2', 2);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_customer_user_2', 3);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_not_login_user_2', 4);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_admin_user_3', 1);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_operator_user_3', 2);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_customer_user_3', 3);
        INSERT INTO "user_user_group" (user_id, user_group_id)
        VALUES ('test_not_login_user_3', 4);
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserUserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserUserGroup {
    Table,
    Id,
    UserId,
    UserGroupId,
}
