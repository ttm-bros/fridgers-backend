use fridgers_backend_domain::user::{Email, UserId, UserName};
use std::marker::PhantomData;

pub struct RegisterUserRequest {
    pub user_id: UserId,
    pub user_name: UserName,
    pub email: Email,
    pub password: String,
    _hide_default_constructor: PhantomData<()>,
}

impl RegisterUserRequest {
    pub fn new(user_id: UserId, user_name: UserName, email: Email, password: String) -> Self {
        Self {
            user_id,
            user_name,
            email,
            password,
            _hide_default_constructor: PhantomData,
        }
    }
}
