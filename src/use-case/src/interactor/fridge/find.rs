use crate::Result;
use crate::dto::fridge::get::{CompartmentWithItems, FridgeWithCompartments};
use crate::interactor::Interactor;
use crate::repository::Repository;

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫IDで冷蔵庫をコンパートメント・アイテムとともに取得する。
    /// 認証済ユーザーが冷蔵庫のオーナーでない場合は NotFound を返す。
    pub async fn handle_get_fridge(
        &self,
        requesting_user_id: &str,
        id: &str,
    ) -> Result<FridgeWithCompartments> {
        let fridge = self.verify_fridge_ownership(requesting_user_id, id).await?;

        let compartments = self.repository.find_compartments_by_fridge_id(id).await?;

        let mut compartments_with_items = Vec::new();
        for compartment in compartments {
            let items = self
                .repository
                .find_items_by_compartment_id(&compartment.id.value().to_string())
                .await?;
            compartments_with_items.push(CompartmentWithItems { compartment, items });
        }

        Ok(FridgeWithCompartments {
            fridge,
            compartments: compartments_with_items,
        })
    }
}
