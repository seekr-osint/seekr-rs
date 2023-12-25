use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "people")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(example = 5u32)]
    pub id: u32, // u64 is unsupported by sqlx

    #[schema(example = "greg")]
    pub firstname: String,
    #[schema(example = "john")]
    pub secondname: Option<String>,
    #[schema(example = "doe")]
    pub lastname: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
