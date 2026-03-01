use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::Result;

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫を削除する
    pub async fn handle_delete_fridge(&self, id: &str) -> Result<()> {
        self.repository.delete_fridge(id).await
    }
}
