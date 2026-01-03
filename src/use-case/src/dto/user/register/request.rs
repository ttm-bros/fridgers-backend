use fridgers_backend_domain::user::{UserId, UserName};
use std::marker::PhantomData;

pub struct RegisterUserRequest {
    pub user_id: UserId,
    pub user_name: UserName,
    _hide_default_constructor: PhantomData<()>,
}

impl RegisterUserRequest {
    pub fn new(user_id: UserId, user_name: UserName) -> Self {
        Self {
            user_id,
            user_name,
            _hide_default_constructor: PhantomData,
        }
    }
}
