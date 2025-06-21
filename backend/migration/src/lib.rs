pub use sea_orm_migration::prelude::*;

mod m20250119_074443_create_user;
mod m20250119_075309_create_role;
mod m20250119_080141_create_user_role;
mod m20250119_081242_create_resource;
mod m20250119_081338_create_permission;
mod m20250119_081745_create_role_resource_permission;
mod m20250621_055742_create_user_group;
mod m20250621_061224_create_user_group_role;
mod m20250621_061502_create_user_user_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250119_074443_create_user::Migration),
            Box::new(m20250119_075309_create_role::Migration),
            Box::new(m20250119_080141_create_user_role::Migration),
            Box::new(m20250119_081242_create_resource::Migration),
            Box::new(m20250119_081338_create_permission::Migration),
            Box::new(m20250119_081745_create_role_resource_permission::Migration),
            Box::new(m20250621_055742_create_user_group::Migration),
            Box::new(m20250621_061224_create_user_group_role::Migration),
            Box::new(m20250621_061502_create_user_user_group::Migration),
        ]
    }
}
