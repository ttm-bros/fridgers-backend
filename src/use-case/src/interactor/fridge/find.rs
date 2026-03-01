use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::{Error, Result};
use fridgers_backend_domain::fridge::Fridge;

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫IDで冷蔵庫を取得する
    pub async fn handle_get_fridge(&self, id: &str) -> Result<Fridge> {
        self.repository
            .find_fridge_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Fridge not found: {}", id)))
    }
}
