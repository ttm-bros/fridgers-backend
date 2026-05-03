use crate::Error;
use crate::Result;
use crate::dto::compartment::create::CreateCompartmentRequest;
use crate::dto::compartment::update::UpdateCompartmentRequest;
use crate::repository::Repository;
use fridgers_backend_domain::compartment::Compartment;

impl<R: Repository> crate::interactor::Interactor<R> {
    pub async fn handle_create_compartment(
        &self,
        request: CreateCompartmentRequest,
    ) -> Result<Compartment> {
        // 冷蔵庫の存在確認
        self.repository
            .find_fridge_by_id(&request.fridge_id.value().to_string())
            .await?
            .ok_or_else(|| {
                Error::NotFound(format!("Fridge not found: {}", request.fridge_id.value()))
            })?;

        let compartment = Compartment::new(request.compartment_id, request.fridge_id, request.name);
        self.repository.save_compartment(&compartment).await?;
        Ok(compartment)
    }

    pub async fn handle_update_compartment(
        &self,
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

        let updated = Compartment::new(request.compartment_id, request.fridge_id, request.name);
        self.repository.update_compartment(&updated).await?;
        Ok(updated)
    }

    pub async fn handle_delete_compartment(
        &self,
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

        self.repository.delete_compartment(compartment_id).await
    }
}
