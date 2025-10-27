use super::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, DeriveIntoActiveModel)]
#[sea_orm(table_name = "trades")]
pub struct Model {
    // Use BigSerial for trades for chronological ordering
    #[sea_orm(primary_key)]
    pub id: i64,

    pub market_id: Uuid,

    // IDs of the two orders that were matched
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,

    // User IDs of the two parties
    pub maker_user_id: Uuid,
    pub taker_user_id: Uuid,

    // Side of the *taker* order
    pub side: String, // "BUY" or "SELL"

    #[sea_orm(column_type = "Decimal(Some((32, 18)))")]
    pub price: Decimal,

    #[sea_orm(column_type = "Decimal(Some((32, 18)))")]
    pub quantity: Decimal,

    #[sea_orm(default_expr = "Now()")]
    pub executed_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Markets",
        from = "Column::MarketId",
        to = "super::markets::Column::Id"
    )]
    Market,

    #[sea_orm(
        belongs_to = "Orders",
        from = "Column::MakerOrderId",
        to = "super::orders::Column::Id"
    )]
    MakerOrder,

    #[sea_orm(
        belongs_to = "Orders",
        from = "Column::TakerOrderId",
        to = "super::orders::Column::Id"
    )]
    TakerOrder,

    #[sea_orm(
        belongs_to = "Users",
        from = "Column::MakerUserId",
        to = "super::users::Column::Id"
    )]
    MakerUser,

    #[sea_orm(
        belongs_to = "Users",
        from = "Column::TakerUserId",
        to = "super::users::Column::Id"
    )]
    TakerUser,
}

// Define related links
impl Related<Markets> for Entity {
    fn to() -> RelationDef {
        Relation::Market.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
