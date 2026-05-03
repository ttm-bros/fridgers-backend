use crate::dto::item::ItemRow;
use crate::repositories::PostgresRepository;
use fridgers_backend_domain::item::Item;
use fridgers_backend_use_case::{Error, Result};
use uuid::Uuid;

impl PostgresRepository {
    pub async fn save_item(&self, item: &Item) -> Result<()> {
        sqlx::query(
            "INSERT INTO items (id, compartment_id, name, quantity, unit, expires_at, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(item.id.value())
        .bind(item.compartment_id.value())
        .bind(item.name.value())
        .bind(item.quantity)
        .bind(item.unit.value())
        .bind(item.expires_at)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to save item: {}", e)))?;
        Ok(())
    }

    pub async fn find_item_by_id(&self, id: &str) -> Result<Option<Item>> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let row = sqlx::query_as::<_, ItemRow>(
            "SELECT id, compartment_id, name, quantity, unit, expires_at, created_at, updated_at \
             FROM items WHERE id = $1",
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find item: {}", e)))?;
        match row {
            Some(row) => Item::try_from(row).map(Some),
            None => Ok(None),
        }
    }

    pub async fn find_items_by_compartment_id(&self, compartment_id: &str) -> Result<Vec<Item>> {
        let uuid = Uuid::parse_str(compartment_id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let rows = sqlx::query_as::<_, ItemRow>(
            "SELECT id, compartment_id, name, quantity, unit, expires_at, created_at, updated_at \
             FROM items WHERE compartment_id = $1 ORDER BY created_at ASC",
        )
        .bind(uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find items: {}", e)))?;
        rows.into_iter().map(Item::try_from).collect()
    }

    pub async fn update_item(&self, item: &Item) -> Result<()> {
        let result = sqlx::query(
            "UPDATE items SET name = $1, quantity = $2, unit = $3, expires_at = $4, updated_at = $5 \
             WHERE id = $6",
        )
        .bind(item.name.value())
        .bind(item.quantity)
        .bind(item.unit.value())
        .bind(item.expires_at)
        .bind(item.updated_at)
        .bind(item.id.value())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to update item: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!(
                "Item not found: {}",
                item.id.value()
            )));
        }
        Ok(())
    }

    pub async fn delete_item(&self, id: &str) -> Result<()> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let result = sqlx::query("DELETE FROM items WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to delete item: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Item not found: {}", id)));
        }
        Ok(())
    }
}
