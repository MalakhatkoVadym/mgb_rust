use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde_json::json;
use tracing::{error, info};

use super::AppState;
use crate::db::CreateRecord;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_records).post(create_record))
        .route(
            "/:id",
            get(get_record).put(update_record).delete(delete_record),
        )
        .with_state(state)
}

async fn create_record(
    State(state): State<AppState>,
    Json(payload): Json<CreateRecord>,
) -> impl IntoResponse {
    info!("Creating new record: {:?}", payload.title);

    match state.db.record_repo().create(payload).await {
        Ok(record) => {
            info!("Record created successfully with id: {}", record.id);
            (StatusCode::CREATED, Json(json!(record)))
        }
        Err(e) => {
            error!("Failed to create record: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create record"})),
            )
        }
    }
}

async fn get_record(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("Fetching record with id: {}", id);

    match state.db.record_repo().get_by_id(id).await {
        Ok(record) => (StatusCode::OK, Json(json!(record))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Record not found"})),
        ),
    }
}

async fn get_all_records(State(state): State<AppState>) -> impl IntoResponse {
    info!("Fetching all records");

    match state.db.record_repo().get_all().await {
        Ok(records) => {
            info!("Retrieved {} records", records.len());
            (StatusCode::OK, Json(json!(records)))
        }
        Err(e) => {
            error!("Failed to fetch records: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch records"})),
            )
        }
    }
}

async fn update_record(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateRecord>,
) -> impl IntoResponse {
    info!("Updating record with id: {}", id);

    match state.db.record_repo().update(id, payload).await {
        Ok(record) => {
            info!("Record updated successfully");
            (StatusCode::OK, Json(json!(record)))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Record not found"})),
        ),
    }
}

async fn delete_record(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("Deleting record with id: {}", id);

    match state.db.record_repo().delete(id).await {
        Ok(_) => {
            info!("Record deleted successfully");
            (StatusCode::NO_CONTENT, Json(json!({})))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Record not found"})),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;

    async fn setup_test_app() -> Router {
        let db = Database::new("sqlite::memory:").await.unwrap();
        let state = AppState { db };
        routes(state)
    }

    #[tokio::test]
    async fn test_create_record_endpoint() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "title": "Test Record",
                            "content": "Test Content"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_all_records_endpoint() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_nonexistent_record() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
