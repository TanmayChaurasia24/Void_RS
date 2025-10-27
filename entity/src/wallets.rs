use super::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveIntoActiveModel, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "wallets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub asset_id: Uuid,

    #[sea_orm(column_type = "Decimal(Some((32,18)))")]
    pub total_balance: Decimal,

    #[sea_orm(column_type = "Decimal(Some((32,18)))")]
    pub available_balance: Decimal,

    #[sea_orm(column_type = "Decimal(Some((32,18)))")]
    pub locked_balance: Decimal,

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
