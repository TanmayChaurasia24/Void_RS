use super::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, DeriveIntoActiveModel)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,
    pub asset_id: Uuid,

    pub transaction_type: String, // "DEPOSIT" or "WITHDRAWAL"
    pub status: String,           // "PENDING", "COMPLETED", "FAILED"

    #[sea_orm(column_type = "Decimal(Some((32, 18)))")]
    pub amount: Decimal,

    #[sea_orm(unique, nullable)]
    pub on_chain_tx_id: Option<String>,

    #[sea_orm(nullable)]
    pub address: Option<String>,

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
        belongs_to = "Assets",
        from = "Column::AssetId",
        to = "super::assets::Column::Id"
    )]
    Asset,
}

// Define related links
impl Related<Users> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<Assets> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
