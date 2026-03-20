mod compartment;
mod fridge;
mod item;
mod user;

use fridgers_backend_domain::compartment::Compartment;
use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_domain::item::Item;
use fridgers_backend_domain::user::User;
use fridgers_backend_use_case::repository::Repository;
use fridgers_backend_use_case::Result;
use sqlx::PgPool;

/// PostgreSQLリポジトリ実装
pub struct PostgresRepository {
    pub(crate) pool: PgPool,
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

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.find_user_by_email(email).await
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

    async fn save_compartment(&self, compartment: &Compartment) -> Result<()> {
        self.save_compartment(compartment).await
    }

    async fn find_compartment_by_id(&self, id: &str) -> Result<Option<Compartment>> {
        self.find_compartment_by_id(id).await
    }

    async fn find_compartments_by_fridge_id(&self, fridge_id: &str) -> Result<Vec<Compartment>> {
        self.find_compartments_by_fridge_id(fridge_id).await
    }

    async fn update_compartment(&self, compartment: &Compartment) -> Result<()> {
        self.update_compartment(compartment).await
    }

    async fn delete_compartment(&self, id: &str) -> Result<()> {
        self.delete_compartment(id).await
    }

    async fn save_item(&self, item: &Item) -> Result<()> {
        self.save_item(item).await
    }

    async fn find_item_by_id(&self, id: &str) -> Result<Option<Item>> {
        self.find_item_by_id(id).await
    }

    async fn find_items_by_compartment_id(&self, compartment_id: &str) -> Result<Vec<Item>> {
        self.find_items_by_compartment_id(compartment_id).await
    }

    async fn update_item(&self, item: &Item) -> Result<()> {
        self.update_item(item).await
    }

    async fn delete_item(&self, id: &str) -> Result<()> {
        self.delete_item(id).await
    }
}
