use std::sync::Arc;

use rocket::tokio::sync::RwLock;

use crate::index::Index;

#[derive(Clone)]
pub struct AppState {
    pub index: Arc<RwLock<Index>>,
}

impl AppState {
    pub fn new(index: Index) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
        }
    }
}
