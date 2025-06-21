use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250119_075309_create_role::Role, m20250621_055742_create_user_group::UserGroup};

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
                    .col(integer(UserGroupRole::UserGroupId))
                    .col(integer(UserGroupRole::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_role_user_group_id")
                            .from(UserGroupRole::Table, UserGroupRole::UserGroupId)
                            .to(UserGroup::Table, UserGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_role_role_id")
                            .from(UserGroupRole::Table, UserGroupRole::RoleId)
                            .to(Role::Table, Role::Id)
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
