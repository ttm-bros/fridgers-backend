use crate::Error;
use crate::Result;
use crate::dto::compartment::create::CreateCompartmentRequest;
use crate::dto::compartment::update::UpdateCompartmentRequest;
use crate::repository::Repository;
use fridgers_backend_domain::compartment::Compartment;

impl<R: Repository> crate::interactor::Interactor<R> {
    pub async fn handle_create_compartment(
        &self,
        requesting_user_id: &str,
        request: CreateCompartmentRequest,
    ) -> Result<Compartment> {
        // 冷蔵庫のオーナー確認（存在＋所有者一致）
        self.verify_fridge_ownership(requesting_user_id, &request.fridge_id.value().to_string())
            .await?;

        let compartment = Compartment::new(request.compartment_id, request.fridge_id, request.name);
        self.repository.save_compartment(&compartment).await?;
        Ok(compartment)
    }

    pub async fn handle_update_compartment(
        &self,
        requesting_user_id: &str,
        request: UpdateCompartmentRequest,
    ) -> Result<Compartment> {
        // コンパートメントの存在確認と所属確認
        let existing = self
            .repository
            .find_compartment_by_id(&request.compartment_id.value().to_string())
            .await?
            .ok_or_else(|| {
                Error::NotFound(format!(
                    "Compartment not found: {}",
                    request.compartment_id.value()
                ))
            })?;

        if existing.fridge_id != request.fridge_id {
            return Err(Error::NotFound(format!(
                "Compartment not found: {}",
                request.compartment_id.value()
            )));
        }

        // 冷蔵庫オーナー確認（パスの fridge_id でチェック）
        self.verify_fridge_ownership(requesting_user_id, &request.fridge_id.value().to_string())
            .await
            .map_err(|_| {
                Error::NotFound(format!(
                    "Compartment not found: {}",
                    request.compartment_id.value()
                ))
            })?;

        let updated = Compartment::new(request.compartment_id, request.fridge_id, request.name);
        self.repository.update_compartment(&updated).await?;
        Ok(updated)
    }

    pub async fn handle_delete_compartment(
        &self,
        requesting_user_id: &str,
        fridge_id: &str,
        compartment_id: &str,
    ) -> Result<()> {
        let existing = self
            .repository
            .find_compartment_by_id(compartment_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Compartment not found: {}", compartment_id)))?;

        if existing.fridge_id.value().to_string() != fridge_id {
            return Err(Error::NotFound(format!(
                "Compartment not found: {}",
                compartment_id
            )));
        }

        // 冷蔵庫オーナー確認
        self.verify_fridge_ownership(requesting_user_id, fridge_id)
            .await
            .map_err(|_| Error::NotFound(format!("Compartment not found: {}", compartment_id)))?;

        self.repository.delete_compartment(compartment_id).await
    }
}
