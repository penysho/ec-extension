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
                    .col(integer(UserRole::UserId))
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRole {
    Table,
    Id,
    UserId,
    RoleId,
}
