use crate::Error;
use crate::Result;
use crate::dto::item::create::CreateItemRequest;
use crate::dto::item::update::UpdateItemRequest;
use crate::repository::Repository;
use chrono::Utc;
use fridgers_backend_domain::item::Item;

impl<R: Repository> crate::interactor::Interactor<R> {
    pub async fn handle_create_item(
        &self,
        requesting_user_id: &str,
        request: CreateItemRequest,
    ) -> Result<Item> {
        // コンパートメントの存在確認＋親冷蔵庫のオーナー確認
        self.verify_compartment_ownership(
            requesting_user_id,
            &request.compartment_id.value().to_string(),
        )
        .await?;

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
        requesting_user_id: &str,
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

        // 親冷蔵庫のオーナー確認（パスの compartment_id 経由）
        self.verify_compartment_ownership(requesting_user_id, compartment_id)
            .await
            .map_err(|_| Error::NotFound(format!("Item not found: {}", request.item_id.value())))?;

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

    pub async fn handle_delete_item(
        &self,
        requesting_user_id: &str,
        compartment_id: &str,
        item_id: &str,
    ) -> Result<()> {
        let existing = self
            .repository
            .find_item_by_id(item_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Item not found: {}", item_id)))?;

        if existing.compartment_id.value().to_string() != compartment_id {
            return Err(Error::NotFound(format!("Item not found: {}", item_id)));
        }

        // 親冷蔵庫のオーナー確認
        self.verify_compartment_ownership(requesting_user_id, compartment_id)
            .await
            .map_err(|_| Error::NotFound(format!("Item not found: {}", item_id)))?;

        self.repository.delete_item(item_id).await
    }
}
