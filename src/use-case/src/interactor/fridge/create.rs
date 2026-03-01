use crate::dto::fridge::create::CreateFridgeRequest;
use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::Result;
use fridgers_backend_domain::fridge::Fridge;

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫を作成する
    pub async fn handle_create_fridge(&self, request: CreateFridgeRequest) -> Result<Fridge> {
        let fridge = Fridge::new(request.fridge_id, request.fridge_name, request.owner_user_id);
        self.repository.save_fridge(&fridge).await?;
        Ok(fridge)
    }
}
