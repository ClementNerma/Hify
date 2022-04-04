use std::sync::Arc;

use rocket::tokio::sync::RwLock;

use crate::index::{Index, SearchIndex};

#[derive(Clone)]
pub struct AppState {
    pub index: Arc<RwLock<Index>>,
    pub search_index: Arc<RwLock<SearchIndex>>,
}

impl AppState {
    pub fn new(index: Index, search_index: SearchIndex) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
            search_index: Arc::new(RwLock::new(search_index)),
        }
    }
}
