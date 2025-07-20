use tokio::sync::RwLock;

use crate::{index::Index, resources::ResourceManager, userdata::UserDataWrapper};

pub struct HttpState {
    pub index: RwLock<Index>,
    pub user_data: RwLock<UserDataWrapper>,
    pub resource_manager: ResourceManager,
}

impl HttpState {
    pub fn new(index: Index, user_data: UserDataWrapper, resource_manager: ResourceManager) -> Self {
        Self {
            index: RwLock::new(index),
            user_data: RwLock::new(user_data),
            resource_manager,
        }
    }
}
