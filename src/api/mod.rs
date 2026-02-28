mod record;

mod artist;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

use crate::db::Database;
use crate::state::StateMachine;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub sm: StateMachine,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/records", record::routes(state.clone()))
        .nest("/artists", artist::routes(state.clone()))
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}
