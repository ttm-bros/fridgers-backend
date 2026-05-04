pub mod auth;
pub mod compartment;
pub mod fridge;
pub mod item;
pub mod user;

use crate::auth::JwtConfig;
use crate::repository::Repository;
use crate::{Error, Result};
use fridgers_backend_domain::compartment::Compartment;
use fridgers_backend_domain::fridge::Fridge;

pub struct Interactor<R: Repository> {
    pub repository: R,
    pub jwt_config: JwtConfig,
}

impl<R: Repository> Interactor<R> {
    pub fn new(repository: R, jwt_config: JwtConfig) -> Self {
        Self {
            repository,
            jwt_config,
        }
    }

    /// 指定した冷蔵庫のオーナーが requesting_user_id と一致することを確認する。
    /// 一致しない場合・冷蔵庫が存在しない場合はいずれも NotFound（リソース存在の漏洩防止）。
    pub(crate) async fn verify_fridge_ownership(
        &self,
        requesting_user_id: &str,
        fridge_id: &str,
    ) -> Result<Fridge> {
        let fridge = self
            .repository
            .find_fridge_by_id(fridge_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Fridge not found: {}", fridge_id)))?;

        if fridge.owner_user_id.value().to_string() != requesting_user_id {
            return Err(Error::NotFound(format!("Fridge not found: {}", fridge_id)));
        }

        Ok(fridge)
    }

    /// 指定したコンパートメントの親冷蔵庫のオーナーが requesting_user_id と一致することを確認する。
    pub(crate) async fn verify_compartment_ownership(
        &self,
        requesting_user_id: &str,
        compartment_id: &str,
    ) -> Result<Compartment> {
        let compartment = self
            .repository
            .find_compartment_by_id(compartment_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Compartment not found: {}", compartment_id)))?;

        self.verify_fridge_ownership(
            requesting_user_id,
            &compartment.fridge_id.value().to_string(),
        )
        .await
        .map_err(|_| Error::NotFound(format!("Compartment not found: {}", compartment_id)))?;

        Ok(compartment)
    }
}
