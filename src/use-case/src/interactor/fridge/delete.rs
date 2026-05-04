use crate::Result;
use crate::interactor::Interactor;
use crate::repository::Repository;

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫を削除する。
    /// 認証済ユーザーが冷蔵庫のオーナーでない場合は NotFound を返す。
    pub async fn handle_delete_fridge(&self, requesting_user_id: &str, id: &str) -> Result<()> {
        self.verify_fridge_ownership(requesting_user_id, id).await?;
        self.repository.delete_fridge(id).await
    }
}
