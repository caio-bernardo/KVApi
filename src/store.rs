use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone, Default)]
pub struct AppState {
    pub db: Arc<RwLock<HashMap<String, String>>>,
}
