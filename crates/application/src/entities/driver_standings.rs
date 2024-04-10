//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "driverStandings")]
pub struct Model {
    #[sea_orm(column_name = "driverStandingsId", primary_key)]
    pub driver_standings_id: i32,
    #[sea_orm(column_name = "raceId")]
    pub race_id: i32,
    #[sea_orm(column_name = "driverId")]
    pub driver_id: i32,
    #[sea_orm(column_type = "Float")]
    pub points: f32,
    pub position: Option<i32>,
    #[sea_orm(column_name = "positionText")]
    pub position_text: Option<String>,
    pub wins: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
