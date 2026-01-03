pub mod user;

use crate::repository::UserRepository;

pub struct Interactor {
    pub repository: Box<dyn UserRepository>,
}

impl Interactor {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }
}
