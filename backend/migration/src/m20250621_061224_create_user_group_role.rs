use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroupRole::Table)
                    .if_not_exists()
                    .col(pk_auto(UserGroupRole::Id))
                    .col(string(UserGroupRole::UserGroupId))
                    .col(string(UserGroupRole::RoleId))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
        INSERT INTO "user_group_role" (user_group_id, role_id)
        VALUES (1, 1);
        INSERT INTO "user_group_role" (user_group_id, role_id)
        VALUES (2, 2);
        INSERT INTO "user_group_role" (user_group_id, role_id)
        VALUES (3, 3);
        INSERT INTO "user_group_role" (user_group_id, role_id)
        VALUES (4, 4);
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroupRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroupRole {
    Table,
    Id,
    UserGroupId,
    RoleId,
}
