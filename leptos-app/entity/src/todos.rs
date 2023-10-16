//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

// use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::{entity::prelude::*, Condition, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Hash)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub due_date: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    // pub created_at: DateTimeUtc,
    // pub updated_at: DateTimeUtc,
}

// Rust type `Option<chrono::datetime::DateTime<chrono::offset::utc::Utc>>` (as SQL type `TIMESTAMPTZ`) is not compatible with SQL type `TIMESTAMP`

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::lists::Entity",
        from = "Column::ListId",
        to = "super::lists::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Lists,
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lists.def()
    }
}

impl Model {
    pub fn calc_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            list_id: uuid::Uuid::new_v4(),
            title: "default".to_string(),
            description: None,
            done: false,
            due_date: None,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(
        list_id: uuid::Uuid,
        title: String,
        description: Option<String>,
        due_date: Option<Date>,
    ) -> Self {
        Self {
            id: Set(uuid::Uuid::new_v4()),
            list_id: Set(list_id),
            title: Set(title),
            description: Set(description),
            due_date: Set(due_date),
            done: Set(false),
            created_at: Set(chrono::Utc::now().naive_local()),
            updated_at: Set(chrono::Utc::now().naive_local()),
        }
    }
}

impl Entity {
    pub fn find_by_id(id: uuid::Uuid) -> Select<Self> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_substring(substring: &str) -> Select<Self> {
        let filter = Condition::any()
            .add(Column::Title.contains(substring))
            .add(Column::Description.contains(substring));

        Self::find().filter(filter)
    }
}
