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

    Ok((StatusCode::CREATED, Json(convert_settings(&created))))
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

    let items: Vec<Value> = result.items.iter().map(convert_settings).collect();

    Ok((headers, Json(items)))
}

pub async fn get_settings_by_id(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    println!("got here");
    let settings = state
        .repository
        .find_by_id(Uuid::from_str(&uuid).unwrap())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(convert_settings(&settings)))
}

pub async fn update_settings(
    State(state): State<Arc<AppState>>,
    Path(uid): Path<Uuid>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
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

    Ok(Json(convert_settings(&updated)))
}

pub async fn delete_settings(
    State(state): State<Arc<AppState>>,
    Path(uid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.repository.delete(uid).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Converts a settings object to JSON
fn convert_settings(settings: &Settings) -> Value {
    let data = match &settings.data {
        Value::Object(map) => map.clone(),
        _ => serde_json::Map::new(),
    };
    let mut obj = serde_json::Map::new();
    obj.insert("id".to_string(), Value::String(settings.id.to_string()));
    obj.insert("data".to_string(), Value::Object(data));
    Value::Object(obj)
}
