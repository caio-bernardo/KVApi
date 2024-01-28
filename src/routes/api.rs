use crate::store::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
pub fn api_routes(state: AppState) -> Router {
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
        Some(_) => (StatusCode::OK, "Item Updated").into_response(),
        None => (StatusCode::CREATED, "Item Created").into_response(),
    }
}

async fn delete_item(State(state): State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let db = &mut state.db.write().unwrap();
    match db.remove(&key) {
        Some(value) => (StatusCode::OK, value).into_response(),
        None => (StatusCode::NOT_FOUND, format!("{} not found", key)).into_response(),
    }
}

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn get_item_found() {
        let (key, value) = ("a", "1");
        let mock_state = AppState::default();
        mock_state
            .db
            .write()
            .unwrap()
            .insert(key.to_string(), value.to_string());

        let req = Request::builder()
            .uri(format!("/{key}"))
            .body(Body::empty())
            .unwrap();
        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::FOUND);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, value)
    }

    #[tokio::test]
    async fn get_item_key_not_found() {
        let mock_state = AppState::default();
        let req = Request::builder().uri("/a").body(Body::empty()).unwrap();
        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, "a not found in db");
    }

    #[tokio::test]
    async fn set_item_create_new() {
        let (key, value) = ("a", "1");
        let mock_state = AppState::default();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/{key}"))
            .body(Body::from(value))
            .unwrap();

        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, "Item Created");
    }

    #[tokio::test]
    async fn set_item_update_old() {
        let (key, old_value, new_value) = ("a", "1", "2");
        let mock_state = AppState::default();
        mock_state
            .db
            .write()
            .unwrap()
            .insert(key.to_string(), old_value.to_string());

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("/{key}"))
            .body(Body::from(new_value))
            .unwrap();
        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, "Item Updated");
    }

    #[tokio::test]
    async fn delete_item_ok() {
        let (key, value) = ("a", "1");
        let mock_state = AppState::default();
        mock_state
            .db
            .write()
            .unwrap()
            .insert(key.to_string(), value.to_string());

        let req = Request::builder()
            .method(Method::DELETE)
            .uri(format!("/{key}"))
            .body(Body::empty())
            .unwrap();
        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, value);
    }

    #[tokio::test]
    async fn delete_item_not_found() {
        let (key, _) = ("a", "1");
        let mock_state = AppState::default();

        let req = Request::builder()
            .method(Method::DELETE)
            .uri(format!("/{key}"))
            .body(Body::empty())
            .unwrap();

        let res = api_routes(mock_state).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body);

        assert_eq!(body, format!("{key} not found"));
    }
}
