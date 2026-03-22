pub mod auth;
pub mod compartment;
pub mod fridge;
pub mod item;
pub mod user;

use crate::auth::JwtConfig;
use crate::repository::Repository;

pub struct Interactor<R: Repository> {
    pub repository: R,
    pub jwt_config: JwtConfig,
}

impl<R: Repository> Interactor<R> {
    pub fn new(repository: R, jwt_config: JwtConfig) -> Self {
        Self {
            repository,
            jwt_config,
        }
    }
}
