use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::settings::{PaginationParams, Settings};
use crate::traits::SettingsRepository;

pub struct AppState {
    pub repository: Arc<dyn SettingsRepository>,
    pub config: Config,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub async fn create_settings(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
    if !body.is_object() {
        return Err(AppError::InvalidJson(
            "Request body must be a JSON object".to_string(),
        ));
    }

    let settings = Settings::new(body);
    let created = state.repository.create(settings).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::to_value(&created).expect("object can be deserialized")),
    ))
}

pub async fn get_all_settings(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = query
        .limit
        .unwrap_or(state.config.default_limit)
        .min(state.config.max_limit);
    let offset = query.offset.unwrap_or(0);

    let params = PaginationParams { limit, offset };
    let result = state.repository.find_all(params).await?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Total-Count", HeaderValue::from(result.total));
    headers.insert("X-Limit", HeaderValue::from(result.limit as u64));
    headers.insert("X-Offset", HeaderValue::from(result.offset as u64));

    let items: Vec<Value> = result
        .items
        .iter()
        .map(|x| serde_json::to_value(x).expect("object can be deserialized"))
        .collect();

    Ok((headers, Json(items)))
}

pub async fn get_settings_by_id(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(uuid) = Uuid::from_str(&uuid) {
        let settings = state
            .repository
            .find_by_id(uuid)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(Json(
            serde_json::to_value(&settings).expect("object can be deserialized"),
        ))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn update_settings(
    State(state): State<Arc<AppState>>,
    Path(uid): Path<String>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(uid) = Uuid::from_str(&uid) {
        if !body.is_object() {
            return Err(AppError::InvalidJson(
                "Request body must be a JSON object".to_string(),
            ));
        }

        let settings = Settings {
            id: uid,
            data: body,
        };
        let updated = state
            .repository
            .update(uid, settings)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(Json(
            serde_json::to_value(&updated).expect("object can be deserialized"),
        ))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn delete_settings(
    State(state): State<Arc<AppState>>,
    Path(uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(uid) = Uuid::from_str(&uid) {
        state.repository.delete(uid).await?;
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    use async_trait::async_trait;
    use serde_json::Value;
    use uuid::Uuid;

    use super::*;
    use crate::error::AppError;
    use crate::settings::PaginatedResult;
    use crate::traits::SettingsRepository;

    pub struct SettingsMock {
        values: RwLock<HashMap<Uuid, Settings>>,
    }

    impl SettingsMock {
        pub fn new() -> Self {
            SettingsMock {
                values: RwLock::new(HashMap::new()),
            }
        }

        pub fn contains(&self, key: Uuid) -> bool {
            self.values.read().unwrap().contains_key(&key)
        }

        pub fn get(&self, key: Uuid) -> Option<Settings> {
            self.values.read().unwrap().get(&key).cloned()
        }

        pub fn get_all(&self) -> Vec<Settings> {
            self.values.read().unwrap().values().cloned().collect()
        }

        pub fn len(&self) -> usize {
            self.values.read().unwrap().len()
        }
    }

    #[async_trait]
    impl SettingsRepository for SettingsMock {
        async fn create(&self, settings: Settings) -> Result<Settings, AppError> {
            let key = Uuid::new_v4();
            self.values.write().unwrap().insert(key, settings.clone());
            Ok(settings)
        }

        async fn find_all(
            &self,
            params: PaginationParams,
        ) -> Result<PaginatedResult<Settings>, AppError> {
            let total = self.values.read().unwrap().len() as u64;
            let mut all_keys: Vec<_> = self.values.read().unwrap().keys().copied().collect();

            all_keys.sort();

            let mut items = Vec::with_capacity(params.limit as usize);
            all_keys
                .iter()
                .skip(params.offset as usize)
                .take(params.limit as usize)
                .for_each(|k| {
                    items.push(
                        self.values
                            .read()
                            .unwrap()
                            .get(k)
                            .expect("key exists")
                            .clone(),
                    )
                });

            Ok(PaginatedResult {
                items,
                total,
                limit: params.limit,
                offset: params.offset,
            })
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Settings>, AppError> {
            let value = self.values.read().unwrap().get(&id).cloned();
            Ok(value)
        }
        async fn update(&self, id: Uuid, settings: Settings) -> Result<Option<Settings>, AppError> {
            let key = Uuid::new_v4();
            if self.values.read().unwrap().contains_key(&id) {
                self.values.write().unwrap().insert(key, settings.clone());
                Ok(Some(settings))
            } else {
                Ok(None)
            }
        }
        async fn delete(&self, id: Uuid) -> Result<(), AppError> {
            self.values.write().unwrap().remove(&id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_settings_works_basic_map() {
        let json_1: Value = serde_json::from_str("{\"foo\": \"bar\"}").unwrap();

        let mock = Arc::new(SettingsMock::new());
        let config = Config::default();
        let state = Arc::new(AppState {
            repository: mock.clone(),
            config,
        });

        create_settings(State(state), Json(json_1.clone()))
            .await
            .unwrap();
        assert_eq!((*mock).len(), 1);
        assert_eq!((*mock).get_all()[0].data, json_1);
    }

    #[tokio::test]
    async fn create_settings_works_empty_json() {
        let json_1: Value = serde_json::from_str("{}").unwrap();

        let mock = Arc::new(SettingsMock::new());
        let config = Config::default();
        let state = Arc::new(AppState {
            repository: mock.clone(),
            config,
        });

        create_settings(State(state), Json(json_1.clone()))
            .await
            .unwrap();
        assert_eq!((*mock).len(), 1);
        assert_eq!((*mock).get_all()[0].data, json_1);
    }
}
