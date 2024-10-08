use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    http::HttpState,
    index::{Index, SearchCache},
};

/// Context shared by all GraphQL queries and mutations
pub struct GraphQLContext {
    pub app_state: Arc<HttpState>,
    pub save_index: SaveIndexFn,
    pub search_cache: Arc<RwLock<SearchCache>>,
}

pub type SaveIndexFn = Box<dyn Fn(&Index) -> Result<(), String> + Send + Sync + 'static>;

impl GraphQLContext {
    pub fn new(app_state: Arc<HttpState>, save_index: SaveIndexFn) -> Self {
        Self {
            app_state,
            save_index,
            search_cache: Arc::new(RwLock::new(SearchCache::new())),
        }
    }
}

#[macro_export]
macro_rules! graphql_index {
    ($ctx_var: ident) => {
        $crate::graphql_ctx_member!($ctx_var, app_state.index, read)
    };
}

#[macro_export]
macro_rules! graphql_user_data {
    ($ctx_var: ident) => {
        $crate::graphql_ctx_member!($ctx_var, app_state.user_data, read)
    };
}

#[macro_export]
macro_rules! graphql_ctx {
    ($ctx_var: ident) => {
        $ctx_var.data::<$crate::graphql::GraphQLContext>().unwrap()
    };
}

#[macro_export]
macro_rules! graphql_ctx_member {
    ($ctx_var: ident, $($member: ident).+, $mode: ident) => {{
        $crate::graphql_ctx!($ctx_var).$($member).+.$mode().await
    }};
}
