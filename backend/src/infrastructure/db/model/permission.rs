//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub action: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_resouce_permission::Entity")]
    RoleResoucePermission,
}

impl Related<super::role_resouce_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleResoucePermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
