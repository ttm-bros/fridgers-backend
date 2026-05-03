use crate::dto::fridge::FridgeRow;
use crate::repositories::PostgresRepository;
use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_use_case::{Error, Result};
use uuid::Uuid;

impl PostgresRepository {
    pub async fn save_fridge(&self, fridge: &Fridge) -> Result<()> {
        sqlx::query("INSERT INTO fridges (id, name, owner_user_id) VALUES ($1, $2, $3)")
            .bind(fridge.id.value())
            .bind(fridge.name.value())
            .bind(fridge.owner_user_id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to save fridge: {}", e)))?;

        Ok(())
    }

    pub async fn find_fridge_by_id(&self, id: &str) -> Result<Option<Fridge>> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let row = sqlx::query_as::<_, FridgeRow>(
            "SELECT id, name, owner_user_id FROM fridges WHERE id = $1",
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find fridge: {}", e)))?;

        match row {
            Some(row) => Fridge::try_from(row).map(Some),
            None => Ok(None),
        }
    }

    pub async fn find_fridges_by_owner_user_id(&self, owner_user_id: &str) -> Result<Vec<Fridge>> {
        let uuid = Uuid::parse_str(owner_user_id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let rows = sqlx::query_as::<_, FridgeRow>(
            "SELECT id, name, owner_user_id FROM fridges WHERE owner_user_id = $1 ORDER BY created_at ASC",
        )
        .bind(uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find fridges: {}", e)))?;

        rows.into_iter().map(Fridge::try_from).collect()
    }

    pub async fn delete_fridge(&self, id: &str) -> Result<()> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let result = sqlx::query("DELETE FROM fridges WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to delete fridge: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Fridge not found: {}", id)));
        }

        Ok(())
    }
}
