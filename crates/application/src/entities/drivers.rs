//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "drivers")]
pub struct Model {
    #[sea_orm(column_name = "driverId", primary_key)]
    pub driver_id: i32,
    #[sea_orm(column_name = "driverRef")]
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<Date>,
    pub nationality: Option<String>,
    #[sea_orm(unique)]
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}