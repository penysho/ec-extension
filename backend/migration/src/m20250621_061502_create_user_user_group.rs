use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250119_074443_create_user::User, m20250621_055742_create_user_group::UserGroup};

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
                    .col(integer(UserUserGroup::UserGroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_user_group_user_id")
                            .from(UserUserGroup::Table, UserUserGroup::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_user_group_user_group_id")
                            .from(UserUserGroup::Table, UserUserGroup::UserGroupId)
                            .to(UserGroup::Table, UserGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
pub(crate) enum UserUserGroup {
    Table,
    Id,
    UserId,
    UserGroupId,
}
