use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{http::HttpState, index::SearchCache};

/// Context shared by all GraphQL queries and mutations
pub struct GraphQLContext {
    pub app_state: Arc<HttpState>,
    pub search_cache: Arc<RwLock<SearchCache>>,
}

impl GraphQLContext {
    pub fn new(app_state: Arc<HttpState>) -> Self {
        Self {
            app_state,
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
macro_rules! graphql_res_manager {
    ($ctx_var: ident) => {
        $crate::graphql_ctx!($ctx_var).app_state.resource_manager
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
