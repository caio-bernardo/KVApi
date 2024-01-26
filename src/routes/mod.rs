use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod api;

pub fn routes() -> Router {
    let state = crate::store::AppState::default();
    Router::new()
        .route("/health", get(health_handler))
        .nest("/api", api::api_routes(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "Hi, I'm healthy!").into_response()
}
