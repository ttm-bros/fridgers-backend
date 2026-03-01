mod fridge_id;
mod fridge_name;

pub use fridge_id::FridgeId;
pub use fridge_name::FridgeName;

use crate::user::UserId;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Fridge {
    pub id: FridgeId,
    pub name: FridgeName,
    pub owner_user_id: UserId,
    _hide_default_constructor: PhantomData<()>,
}

impl Fridge {
    pub fn new(id: FridgeId, name: FridgeName, owner_user_id: UserId) -> Self {
        Self {
            id,
            name,
            owner_user_id,
            _hide_default_constructor: PhantomData,
        }
    }
}
