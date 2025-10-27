// define trading pairs sol_usdc

use super::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, DeriveIntoActiveModel)]
#[sea_orm(table_name = "markets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub symbol: String,

    pub base_asset_id: Uuid,

    pub quote_asset_id: Uuid,

    pub status: String, // e.g., "active", "maintenance", "inactive"

    #[sea_orm(default_expr = "Now()")]
    pub created_at: ChronoDateTimeUtc,

    #[sea_orm(default_expr = "Now()")]
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Assets",
        from = "Column::BaseAssetId",
        to = "super::assets::Column::Id"
    )]
    BaseAsset,

    #[sea_orm(
        belongs_to = "Assets",
        from = "Column::QuoteAssetId",
        to = "super::assets::Column::Id"
    )]
    QuoteAsset,

    #[sea_orm(has_many = "Orders")]
    Orders,
}

impl Related<Orders> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<Assets> for Entity {
    fn to() -> RelationDef {
        panic!("Relation not found")
    }
}

// 2 explicit relationship linkers
pub struct MarketToBaseAsset;
impl Linked for MarketToBaseAsset {
    type FromEntity = Entity;
    type ToEntity = Assets;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::BaseAsset.def()]
    }
}

pub struct MarketToQuoteAsset;
impl Linked for MarketToQuoteAsset {
    type FromEntity = Entity;
    type ToEntity = Assets;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::QuoteAsset.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
