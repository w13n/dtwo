use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub id: Uuid,
    #[serde(flatten)]
    pub data: Value,
}

impl Settings {
    pub fn new(data: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            data,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
}
