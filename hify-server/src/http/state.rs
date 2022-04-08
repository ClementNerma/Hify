use std::sync::Arc;

use rocket::tokio::sync::RwLock;

use crate::index::{Index, SearchCache};

#[derive(Clone)]
pub struct AppState {
    pub index: Arc<RwLock<Index>>,
    pub search_cache: Arc<RwLock<SearchCache>>,
}

impl AppState {
    pub fn new(index: Index) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
            search_cache: Arc::new(RwLock::new(SearchCache::new())),
        }
    }
}
