pub use sea_orm_migration::prelude::*;

mod m20250119_074443_create_user;
mod m20250119_075309_create_role;
mod m20250119_080141_create_user_role;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250119_074443_create_user::Migration),
            Box::new(m20250119_075309_create_role::Migration),
            Box::new(m20250119_080141_create_user_role::Migration),
        ]
    }
}
