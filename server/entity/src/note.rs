//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "note"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub note: String,
    pub duration_hours: i32,
    pub manual_password: Option<String>,
    pub notify_email: Option<String>,
    pub destroy_without_confirmation: bool,
    pub created_at: DateTime,
    pub delete_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Note,
    DurationHours,
    ManualPassword,
    NotifyEmail,
    DestroyWithoutConfirmation,
    CreatedAt,
    DeleteAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Uuid.def(),
            Self::Note => ColumnType::String(None).def(),
            Self::DurationHours => ColumnType::Integer.def(),
            Self::ManualPassword => ColumnType::String(Some(1000u32)).def().null(),
            Self::NotifyEmail => ColumnType::String(Some(1000u32)).def().null(),
            Self::DestroyWithoutConfirmation => ColumnType::Boolean.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::DeleteAt => ColumnType::DateTime.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
