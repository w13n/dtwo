use std::time;

use async_trait::async_trait;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use uuid::Uuid;

use crate::error::AppError;
use crate::settings::{PaginatedResult, PaginationParams, Settings};

use super::traits::SettingsRepository;

pub struct SqliteSettingsRepository {
    pool: SqlitePool,
}

impl SqliteSettingsRepository {
    pub async fn new(database_path: &str) -> Result<Self, AppError> {
        // Ensure the directory exists
        if let Some(parent) = std::path::Path::new(database_path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AppError::Internal(format!("Failed to create database directory: {}", e))
            })?;
        }

        let database_url = format!("sqlite:{}?mode=rwc", database_path);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let repo = Self { pool };
        repo.run_migrations().await?;

        Ok(repo)
    }

    async fn run_migrations(&self) -> Result<(), AppError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                id TEXT PRIMARY KEY NOT NULL,
                data TEXT NOT NULL,
                time INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn create(&self, settings: Settings) -> Result<Settings, AppError> {
        let id_str = settings.id.to_string();
        let data_str = serde_json::to_string(&settings.data)
            .map_err(|e| AppError::Internal(format!("Failed to serialize JSON: {}", e)))?;

        let time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        sqlx::query("INSERT INTO settings (id, data, time) VALUES (?, ?, ?)")
            .bind(&id_str)
            .bind(&data_str)
            .bind(&time)
            .execute(&self.pool)
            .await?;

        Ok(settings)
    }

    async fn find_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResult<Settings>, AppError> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings")
            .fetch_one(&self.pool)
            .await?;

        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT id, data FROM settings ORDER BY time DESC LIMIT ? OFFSET ?")
                .bind(params.limit as i64)
                .bind(params.offset as i64)
                .fetch_all(&self.pool)
                .await?;

        let items: Result<Vec<Settings>, AppError> = rows
            .into_iter()
            .map(|(id_str, data_str)| {
                let id = Uuid::parse_str(&id_str)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID in database: {}", e)))?;
                let data: serde_json::Value = serde_json::from_str(&data_str)
                    .map_err(|e| AppError::Internal(format!("Invalid JSON in database: {}", e)))?;
                Ok(Settings { id, data })
            })
            .collect();

        Ok(PaginatedResult {
            items: items?,
            total: total.0 as u64,
            limit: params.limit,
            offset: params.offset,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Settings>, AppError> {
        let id_str = id.to_string();

        let row: Option<(String, String)> =
            sqlx::query_as("SELECT id, data FROM settings WHERE id = ?")
                .bind(&id_str)
                .fetch_optional(&self.pool)
                .await?;

        match row {
            Some((id_str, data_str)) => {
                let id = Uuid::parse_str(&id_str)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID in database: {}", e)))?;
                let data: serde_json::Value = serde_json::from_str(&data_str)
                    .map_err(|e| AppError::Internal(format!("Invalid JSON in database: {}", e)))?;
                Ok(Some(Settings { id, data }))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, id: Uuid, settings: Settings) -> Result<Option<Settings>, AppError> {
        let id_str = id.to_string();
        let data_str = serde_json::to_string(&settings.data)
            .map_err(|e| AppError::Internal(format!("Failed to serialize JSON: {}", e)))?;
        let time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let result = sqlx::query("UPDATE settings SET data = ?, time = ? WHERE id = ?")
            .bind(&data_str)
            .bind(&time)
            .bind(&id_str)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Ok(None)
        } else {
            Ok(Some(Settings {
                id,
                data: settings.data,
            }))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let id_str = id.to_string();

        sqlx::query("DELETE FROM settings WHERE id = ?")
            .bind(&id_str)
            .execute(&self.pool)
            .await?;

        // Always return success for idempotency
        Ok(())
    }
}
