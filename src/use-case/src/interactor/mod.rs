pub mod user;

use crate::repository::UserRepository;

pub struct FridgersRestInteractor {
    pub repository: Box<dyn UserRepository>,
}

impl FridgersRestInteractor {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }
}
