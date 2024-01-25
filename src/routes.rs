use crate::store::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn routes() -> Router {
    let state = AppState::default();
    Router::new()
        .route("/health", get(health_handler))
        .nest("/api", api_routes(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hi, I'm healthy!").into_response()
}

fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_item))
        .route("/:key", get(get_item).post(set_item).delete(delete_item))
        .with_state(state)
}

async fn list_item(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.read().unwrap().clone();
    Json(db)
}

async fn get_item(State(state): State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let db = &state.db.read().unwrap();
    match db.get(&key) {
        Some(value) => (StatusCode::FOUND, value.to_owned()).into_response(),
        None => (StatusCode::NOT_FOUND, format!("{} not found in db", &key)).into_response(),
    }
}

async fn set_item(
    State(state): State<AppState>,
    Path(key): Path<String>,
    value: String,
) -> impl IntoResponse {
    let db = &mut state.db.write().unwrap();
    match db.insert(key, value) {
        Some(_) => (StatusCode::OK, "Updated Successfuly").into_response(),
        None => (StatusCode::CREATED, "Created Successfuly").into_response(),
    }
}

async fn delete_item(State(state): State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let db = &mut state.db.write().unwrap();
    match db.remove(&key) {
        Some(value) => (StatusCode::OK, format!("{} deleted successfuly", value)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            format!("{} not found in database", key),
        )
            .into_response(),
    }
}
