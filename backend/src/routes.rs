use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::{
    AppState, create_settings, delete_settings, get_all_settings, get_settings_by_id,
    update_settings,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/settings", post(create_settings))
        .route("/settings", get(get_all_settings))
        .route("/settings/{uid}", get(get_settings_by_id))
        .route("/settings/{uid}", put(update_settings))
        .route("/settings/{uid}", delete(delete_settings))
        .with_state(state)
}
