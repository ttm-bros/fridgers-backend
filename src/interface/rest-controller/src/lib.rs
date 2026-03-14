mod handler;
pub mod router;
pub mod schema;
mod error;

// ルーティング設定を公開
pub use router::compartment::configure as configure_compartments;
pub use router::fridge::configure as configure_fridges;
pub use router::health::configure as configure_health;
pub use router::item::configure as configure_items;
pub use router::user::configure as configure_users;
