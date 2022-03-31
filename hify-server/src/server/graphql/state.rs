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

#[macro_export]
macro_rules! graphql_index {
    ($ctx_var: ident) => {{
        let ctx = $ctx_var.data::<GraphQLContext>().unwrap();
        ctx.index.read().await
    }};
}
