use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{create_note_handler, health_check_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/notes", post(create_note_handler))
        .with_state(app_state)
}
