use fridgers_backend_domain::user::{User, UserId, UserName};
use fridgers_backend_use_case::repository::Repository;
use fridgers_backend_use_case::{Error, Result};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// PostgreSQLリポジトリ実装
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// SQLの結果をマッピングするための内部構造体
#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
}

impl Repository for PostgresRepository {
    async fn save_user(&self, user: &User) -> Result<()> {
        sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
            .bind(user.id.value())
            .bind(user.name.value())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to save user: {}", e)))?;

        Ok(())
    }

    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;

        let row = sqlx::query_as::<_, UserRow>("SELECT id, name FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ExternalServer(format!("Failed to find user: {}", e)))?;

        match row {
            Some(row) => {
                let user_id = UserId::from(row.id);
                let user_name = UserName::try_from(row.name).map_err(Error::from)?;
                Ok(Some(User::new(user_id, user_name)))
            }
            None => Ok(None),
        }
    }

    async fn delete_user(&self, id: &str) -> Result<()> {
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
