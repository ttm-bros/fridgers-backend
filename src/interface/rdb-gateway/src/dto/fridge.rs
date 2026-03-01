use fridgers_backend_domain::fridge::{Fridge, FridgeId, FridgeName};
use fridgers_backend_domain::user::UserId;
use fridgers_backend_use_case::Error;
use sqlx::FromRow;
use uuid::Uuid;

/// fridgesテーブルの行をマッピングするDTO
#[derive(FromRow)]
pub struct FridgeRow {
    pub id: Uuid,
    pub name: String,
    pub owner_user_id: Uuid,
}

impl TryFrom<FridgeRow> for Fridge {
    type Error = Error;

    fn try_from(row: FridgeRow) -> Result<Self, Self::Error> {
        let fridge_id = FridgeId::from(row.id);
        let fridge_name = FridgeName::try_from(row.name)?;
        let owner_user_id = UserId::from(row.owner_user_id);
        Ok(Fridge::new(fridge_id, fridge_name, owner_user_id))
    }
}
