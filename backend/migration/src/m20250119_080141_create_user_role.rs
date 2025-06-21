use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250119_074443_create_user::User, m20250119_075309_create_role::Role};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(pk_auto(UserRole::Id))
                    .col(string(UserRole::UserId))
                    .col(integer(UserRole::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_user_id")
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_role_id")
                            .from(UserRole::Table, UserRole::RoleId)
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
        INSERT INTO "user_role" (user_id, role_id)
        VALUES ('test_admin_user_1', 1);
        INSERT INTO "user_role" (user_id, role_id)
        VALUES ('test_operator_user_1', 2);
        INSERT INTO "user_role" (user_id, role_id)
        VALUES ('test_customer_user_1', 3);
        INSERT INTO "user_role" (user_id, role_id)
        VALUES ('test_not_login_user_1', 4);
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum UserRole {
    Table,
    Id,
    UserId,
    RoleId,
}
