use crate::dto::user::UserRow;
use crate::repositories::PostgresRepository;
use fridgers_backend_domain::user::User;
use fridgers_backend_use_case::{Error, Result};
use uuid::Uuid;

impl PostgresRepository {
    pub async fn save_user(&self, user: &User) -> Result<()> {
        sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
            .bind(user.id.value())
            .bind(user.name.value())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to save user: {}", e)))?;

        Ok(())
    }

    pub async fn find_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let row = sqlx::query_as::<_, UserRow>("SELECT id, name FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to find user: {}", e)))?;

        match row {
            Some(row) => User::try_from(row).map(Some),
            None => Ok(None),
        }
    }

    pub async fn delete_user(&self, id: &str) -> Result<()> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to delete user: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("User not found: {}", id)));
        }

        Ok(())
    }
}
