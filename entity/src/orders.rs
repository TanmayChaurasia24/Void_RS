use super::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, DeriveIntoActiveModel)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,
    pub market_id: Uuid,

    pub side: String,       // "BUY" or "SELL"
    pub order_type: String, // "LIMIT" or "MARKET"
    pub status: String,     // "OPEN", "PARTIALLY_FILLED", "FILLED", "CANCELED"

    // Price is nullable for MARKET orders
    #[sea_orm(column_type = "Decimal(Some((32, 18)))", nullable)]
    pub price: Option<Decimal>,

    #[sea_orm(column_type = "Decimal(Some((32, 18)))")]
    pub quantity: Decimal,

    #[sea_orm(column_type = "Decimal(Some((32, 18)))", default_value = "0.0")]
    pub filled_quantity: Decimal,

    #[sea_orm(default_expr = "Now()")]
    pub created_at: ChronoDateTimeUtc,

    #[sea_orm(default_expr = "Now()")]
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Users",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,

    #[sea_orm(
        belongs_to = "Markets",
        from = "Column::MarketId",
        to = "super::markets::Column::Id"
    )]
    Market,
}

// Define related links
impl Related<Users> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<Markets> for Entity {
    fn to() -> RelationDef {
        Relation::Market.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
