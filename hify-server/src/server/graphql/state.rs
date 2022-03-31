use std::sync::Arc;

use rocket::tokio::sync::RwLock;

use crate::index::Index;

pub struct GraphQLContext {
    pub index: Arc<RwLock<Index>>,
}

impl GraphQLContext {
    pub fn new(index: Index) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
        }
    }
}
