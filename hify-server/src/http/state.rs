use tokio::sync::RwLock;

use crate::{
    index::{Index, SearchCache},
    resources::ResourceManager,
    userdata::UserDataWrapper,
};

pub struct AppState {
    pub index: RwLock<Index>,
    pub user_data: RwLock<UserDataWrapper>,
    pub search_cache: RwLock<SearchCache>,
    pub resource_manager: ResourceManager,
}

impl AppState {
    pub fn new(
        index: Index,
        user_data: UserDataWrapper,
        resource_manager: ResourceManager,
    ) -> Self {
        Self {
            index: RwLock::new(index),
            user_data: RwLock::new(user_data),
            search_cache: RwLock::new(SearchCache::new()),
            resource_manager,
        }
    }
}
