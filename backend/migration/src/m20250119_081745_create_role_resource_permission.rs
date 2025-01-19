use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250119_075309_create_role::Role, m20250119_081242_create_resource::Resource,
    m20250119_081338_create_permission::Permission,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleResoucePermission::Table)
                    .if_not_exists()
                    .col(pk_auto(RoleResoucePermission::Id))
                    .col(integer(RoleResoucePermission::RoleId))
                    .col(integer(RoleResoucePermission::ResourceId))
                    .col(integer(RoleResoucePermission::PermissionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_role")
                            .from(RoleResoucePermission::Table, RoleResoucePermission::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_resource")
                            .from(
                                RoleResoucePermission::Table,
                                RoleResoucePermission::ResourceId,
                            )
                            .to(Resource::Table, Resource::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_permission")
                            .from(
                                RoleResoucePermission::Table,
                                RoleResoucePermission::PermissionId,
                            )
                            .to(Permission::Table, Permission::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleResoucePermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum RoleResoucePermission {
    Table,
    Id,
    RoleId,
    ResourceId,
    PermissionId,
}
