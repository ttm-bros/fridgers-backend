use std::marker::PhantomData;

pub struct LoginRequest {
    pub email: String,
    pub password: String,
    _hide_default_constructor: PhantomData<()>,
}

impl LoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password,
            _hide_default_constructor: PhantomData,
        }
    }
}
