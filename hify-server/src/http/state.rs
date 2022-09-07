use std::sync::Arc;

use rocket::tokio::sync::RwLock;

use crate::{
    index::{Index, SearchCache},
    userdata::UserData,
};

#[derive(Clone)]
pub struct AppState {
    pub index: Arc<RwLock<Index>>,
    pub user_data: Arc<RwLock<UserData>>,
    pub search_cache: Arc<RwLock<SearchCache>>,
}

impl AppState {
    pub fn new(index: Index, user_data: UserData) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
            user_data: Arc::new(RwLock::new(user_data)),
            search_cache: Arc::new(RwLock::new(SearchCache::new())),
        }
    }
}
