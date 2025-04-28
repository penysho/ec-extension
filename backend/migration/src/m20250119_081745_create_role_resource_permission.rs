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
                    .table(RoleResourcePermission::Table)
                    .if_not_exists()
                    .col(pk_auto(RoleResourcePermission::Id))
                    .col(integer(RoleResourcePermission::RoleId))
                    .col(integer(RoleResourcePermission::ResourceId))
                    .col(integer(RoleResourcePermission::PermissionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_role")
                            .from(
                                RoleResourcePermission::Table,
                                RoleResourcePermission::RoleId,
                            )
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_resource")
                            .from(
                                RoleResourcePermission::Table,
                                RoleResourcePermission::ResourceId,
                            )
                            .to(Resource::Table, Resource::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_resource_permission_permission")
                            .from(
                                RoleResourcePermission::Table,
                                RoleResourcePermission::PermissionId,
                            )
                            .to(Permission::Table, Permission::Id)
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
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (1, 1, 1, 1);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (2, 1, 2, 1);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (3, 1, 3, 1);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (4, 1, 4, 1);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (5, 1, 5, 1);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (6, 2, 1, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (7, 2, 1, 6);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (8, 2, 1, 7);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (9, 2, 2, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (10, 2, 2, 6);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (11, 2, 2, 7);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (12, 2, 3, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (13, 2, 3, 6);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (14, 2, 3, 7);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (15, 2, 4, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (16, 2, 4, 6);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (17, 2, 4, 7);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (18, 2, 5, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (19, 2, 5, 6);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (20, 2, 5, 7);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (21, 3, 1, 5);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (22, 3, 2, 2);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (22, 3, 2, 3);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (22, 3, 3, 2);
        INSERT INTO "role_resource_permission" (id, role_id, resource_id, permission_id)
        VALUES (22, 3, 3, 3);
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(RoleResourcePermission::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum RoleResourcePermission {
    Table,
    Id,
    RoleId,
    ResourceId,
    PermissionId,
}
