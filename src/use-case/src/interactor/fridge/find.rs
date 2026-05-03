use crate::dto::fridge::get::{CompartmentWithItems, FridgeWithCompartments};
use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::{Error, Result};

impl<R: Repository> Interactor<R> {
    /// 冷蔵庫IDで冷蔵庫をコンパートメント・アイテムとともに取得する
    pub async fn handle_get_fridge(&self, id: &str) -> Result<FridgeWithCompartments> {
        let fridge = self
            .repository
            .find_fridge_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Fridge not found: {}", id)))?;

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
