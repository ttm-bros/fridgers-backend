use crate::dto::item::create::CreateItemRequest;
use crate::dto::item::update::UpdateItemRequest;
use crate::repository::Repository;
use crate::Result;
use crate::Error;
use chrono::Utc;
use fridgers_backend_domain::item::Item;

impl<R: Repository> crate::interactor::Interactor<R> {
    pub async fn handle_create_item(&self, request: CreateItemRequest) -> Result<Item> {
        // コンパートメントの存在確認
        self.repository
            .find_compartment_by_id(&request.compartment_id.value().to_string())
            .await?
            .ok_or_else(|| {
                Error::NotFound(format!(
                    "Compartment not found: {}",
                    request.compartment_id.value()
                ))
            })?;

        let now = Utc::now();
        let item = Item::new(
            request.item_id,
            request.compartment_id,
            request.name,
            request.quantity,
            request.unit,
            request.expires_at,
            now,
            now,
        );
        self.repository.save_item(&item).await?;
        Ok(item)
    }

    pub async fn handle_update_item(
        &self,
        compartment_id: &str,
        request: UpdateItemRequest,
    ) -> Result<Item> {
        let existing = self
            .repository
            .find_item_by_id(&request.item_id.value().to_string())
            .await?
            .ok_or_else(|| {
                Error::NotFound(format!("Item not found: {}", request.item_id.value()))
            })?;

        if existing.compartment_id.value().to_string() != compartment_id {
            return Err(Error::NotFound(format!(
                "Item not found: {}",
                request.item_id.value()
            )));
        }

        let updated = Item::new(
            request.item_id,
            existing.compartment_id,
            request.name,
            request.quantity,
            request.unit,
            request.expires_at,
            existing.created_at,
            Utc::now(),
        );
        self.repository.update_item(&updated).await?;
        Ok(updated)
    }

    pub async fn handle_delete_item(&self, compartment_id: &str, item_id: &str) -> Result<()> {
        let existing = self
            .repository
            .find_item_by_id(item_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Item not found: {}", item_id)))?;

        if existing.compartment_id.value().to_string() != compartment_id {
            return Err(Error::NotFound(format!("Item not found: {}", item_id)));
        }

        self.repository.delete_item(item_id).await
    }
}
