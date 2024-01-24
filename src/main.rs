#![allow(unused_variables)]
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use axum::{extract::State, Json};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new()
        .route("/hello", get(hello_handler))
        .nest("/api", routes_api());

    info!("Starting Server...Listening on https://0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello_handler() -> impl IntoResponse {
    info!("GET /hello");
    (StatusCode::OK, "Hello, Everything appears to be working!").into_response()
}

#[derive(Clone)]
struct AppState {
    db: HashMap<String, String>,
}

impl AppState {
    fn new() -> Self {
        let db = HashMap::new();
        AppState { db }
    }
}

fn routes_api() -> Router {
    let state = Arc::new(RwLock::new(AppState::new()));
    Router::new()
        .route("/", get(list_item))
        .route("/:key", get(get_item))
        .route("/:key/:value", post(set_item))
        .route("/:key", delete(delete_item))
        .with_state(state)
}

async fn list_item(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    info!("GET /api/");
    let db = &state.read().unwrap().db;
    Json(db.clone())
}

async fn get_item(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    info!("GET /api/:key");
    let db = &state.read().unwrap().db;
    match db.get(&key) {
        Some(value) => (StatusCode::FOUND, value.to_owned()).into_response(),
        None => (StatusCode::NOT_FOUND, format!("{} not found in db", &key)).into_response(),
    }
}

async fn set_item(
    State(state): State<Arc<RwLock<AppState>>>,
    Path((key, value)): Path<(String, String)>,
) -> impl IntoResponse {
    info!("POST /api/:key/:value");
    let db = &mut state.write().unwrap().db;
    match db.insert(key, value) {
        Some(value) => (StatusCode::OK, "Updated Successfuly").into_response(),
        None => (StatusCode::CREATED, "Created Successfuly").into_response(),
    }
}

async fn delete_item(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    info!("DELETE /api/:key");
    let db = &mut state.write().unwrap().db;
    match db.remove(&key) {
        Some(value) => (StatusCode::OK, format!("{} deleted successfuly", value)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            format!("{} not found in database", key),
        )
            .into_response(),
    }
}
