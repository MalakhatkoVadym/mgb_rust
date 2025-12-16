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
use crate::db::CreateArtist;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_artists).post(create_artist))
        .route(
            "/:id",
            get(get_artist).put(update_artist).delete(delete_artist),
        )
        .with_state(state)
}

async fn create_artist(
    State(state): State<AppState>,
    Json(payload): Json<CreateArtist>,
) -> impl IntoResponse {
    info!("Creating new artist: {:?}", payload.name);

    match state.db.artist_repo().create(payload).await {
        Ok(artist) => {
            info!("Artist created successfully with id: {}", artist.id);
            (StatusCode::CREATED, Json(json!(artist)))
        }
        Err(e) => {
            error!("Failed to create artist: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create artist"})),
            )
        }
    }
}

async fn get_artist(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("Fetching artist with id: {}", id);

    match state.db.artist_repo().get_by_id(id).await {
        Ok(artist) => (StatusCode::OK, Json(json!(artist))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Artist not found"})),
        ),
    }
}

async fn get_all_artists(State(state): State<AppState>) -> impl IntoResponse {
    info!("Fetching all artists");

    match state.db.artist_repo().get_all().await {
        Ok(artists) => {
            info!("Retrieved {} artists", artists.len());
            (StatusCode::OK, Json(json!(artists)))
        }
        Err(e) => {
            error!("Failed to fetch artists: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch artists"})),
            )
        }
    }
}

async fn update_artist(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateArtist>,
) -> impl IntoResponse {
    info!("Updating artist with id: {}", id);

    match state.db.artist_repo().update(id, payload).await {
        Ok(artist) => {
            info!("Artist updated successfully");
            (StatusCode::OK, Json(json!(artist)))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Artist not found"})),
        ),
    }
}

async fn delete_artist(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("Deleting artist with id: {}", id);

    match state.db.artist_repo().delete(id).await {
        Ok(_) => {
            info!("Artist deleted successfully");
            (StatusCode::NO_CONTENT, Json(json!({})))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Artist not found"})),
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
    async fn test_create_artist_endpoint() {
        let app = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "name": "Test Artist",
                            "genre": "Rock"
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
    async fn test_get_all_artists_endpoint() {
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
    async fn test_get_nonexistent_artist() {
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
