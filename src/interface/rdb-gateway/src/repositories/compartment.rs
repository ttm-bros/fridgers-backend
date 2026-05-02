use crate::dto::compartment::CompartmentRow;
use crate::repositories::PostgresRepository;
use fridgers_backend_domain::compartment::Compartment;
use fridgers_backend_use_case::{Error, Result};
use uuid::Uuid;

impl PostgresRepository {
    pub async fn save_compartment(&self, compartment: &Compartment) -> Result<()> {
        sqlx::query("INSERT INTO compartments (id, fridge_id, name) VALUES ($1, $2, $3)")
            .bind(compartment.id.value())
            .bind(compartment.fridge_id.value())
            .bind(compartment.name.value())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to save compartment: {}", e)))?;
        Ok(())
    }

    pub async fn find_compartment_by_id(&self, id: &str) -> Result<Option<Compartment>> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let row = sqlx::query_as::<_, CompartmentRow>(
            "SELECT id, fridge_id, name FROM compartments WHERE id = $1",
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find compartment: {}", e)))?;
        match row {
            Some(row) => Compartment::try_from(row).map(Some),
            None => Ok(None),
        }
    }

    pub async fn find_compartments_by_fridge_id(
        &self,
        fridge_id: &str,
    ) -> Result<Vec<Compartment>> {
        let uuid = Uuid::parse_str(fridge_id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let rows = sqlx::query_as::<_, CompartmentRow>(
            "SELECT id, fridge_id, name FROM compartments WHERE fridge_id = $1 ORDER BY created_at ASC",
        )
        .bind(uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::ExternalServer(format!("Failed to find compartments: {}", e)))?;
        rows.into_iter().map(Compartment::try_from).collect()
    }

    pub async fn update_compartment(&self, compartment: &Compartment) -> Result<()> {
        let result =
            sqlx::query("UPDATE compartments SET name = $1, updated_at = NOW() WHERE id = $2")
                .bind(compartment.name.value())
                .bind(compartment.id.value())
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    Error::ExternalServer(format!("Failed to update compartment: {}", e))
                })?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!(
                "Compartment not found: {}",
                compartment.id.value()
            )));
        }
        Ok(())
    }

    pub async fn delete_compartment(&self, id: &str) -> Result<()> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
        let result = sqlx::query("DELETE FROM compartments WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to delete compartment: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Compartment not found: {}", id)));
        }
        Ok(())
    }
}
