use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppError;
use crate::settings::{PaginatedResult, PaginationParams, Settings};

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn create(&self, settings: Settings) -> Result<Settings, AppError>;
    async fn find_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResult<Settings>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Settings>, AppError>;
    async fn update(&self, id: Uuid, settings: Settings) -> Result<Option<Settings>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
