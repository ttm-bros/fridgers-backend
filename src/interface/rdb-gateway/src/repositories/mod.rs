mod fridge;
mod user;

use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_domain::user::User;
use fridgers_backend_use_case::repository::Repository;
use fridgers_backend_use_case::Result;
use sqlx::PgPool;

/// PostgreSQLリポジトリ実装
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository for PostgresRepository {
    async fn save_user(&self, user: &User) -> Result<()> {
        self.save_user(user).await
    }

    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>> {
        self.find_user_by_id(id).await
    }

    async fn delete_user(&self, id: &str) -> Result<()> {
        self.delete_user(id).await
    }

    async fn save_fridge(&self, fridge: &Fridge) -> Result<()> {
        self.save_fridge(fridge).await
    }

    async fn find_fridge_by_id(&self, id: &str) -> Result<Option<Fridge>> {
        self.find_fridge_by_id(id).await
    }

    async fn delete_fridge(&self, id: &str) -> Result<()> {
        self.delete_fridge(id).await
    }
}
