use crate::Result;
use crate::dto::fridge::list::ListFridgesResponse;
use crate::interactor::Interactor;
use crate::repository::Repository;

impl<R: Repository> Interactor<R> {
    /// オーナーユーザーIDに紐づく冷蔵庫一覧を取得する
    pub async fn handle_list_fridges(&self, owner_user_id: &str) -> Result<ListFridgesResponse> {
        let fridges = self
            .repository
            .find_fridges_by_owner_user_id(owner_user_id)
            .await?;
        Ok(ListFridgesResponse { fridges })
    }
}
