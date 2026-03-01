use fridgers_backend_domain::fridge::{FridgeId, FridgeName};
use fridgers_backend_domain::user::UserId;
use std::marker::PhantomData;

pub struct CreateFridgeRequest {
    pub fridge_id: FridgeId,
    pub fridge_name: FridgeName,
    pub owner_user_id: UserId,
    _hide_default_constructor: PhantomData<()>,
}

impl CreateFridgeRequest {
    pub fn new(fridge_id: FridgeId, fridge_name: FridgeName, owner_user_id: UserId) -> Self {
        Self {
            fridge_id,
            fridge_name,
            owner_user_id,
            _hide_default_constructor: PhantomData,
        }
    }
}
