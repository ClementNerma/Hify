use std::{path::PathBuf, sync::Arc};

use tokio::sync::RwLock;

use crate::{index::Index, resources::ResourceManager, userdata::UserDataWrapper};

pub struct HttpState {
    pub music_dir: PathBuf,
    pub index: RwLock<Index>,
    pub user_data: RwLock<UserDataWrapper>,
    pub resource_manager: Arc<ResourceManager>,
}

impl HttpState {
    pub fn new(
        music_dir: PathBuf,
        index: Index,
        user_data: UserDataWrapper,
        resource_manager: ResourceManager,
    ) -> Self {
        Self {
            music_dir,
            index: RwLock::new(index),
            user_data: RwLock::new(user_data),
            resource_manager: Arc::new(resource_manager),
        }
    }
}
