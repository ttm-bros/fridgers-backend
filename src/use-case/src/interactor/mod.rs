pub mod compartment;
pub mod fridge;
pub mod item;
pub mod user;

use crate::repository::Repository;

pub struct Interactor<R: Repository> {
    pub repository: R,
}

impl<R: Repository> Interactor<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}
