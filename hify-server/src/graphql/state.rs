use crate::{http::AppState, index::Index};

pub struct GraphQLContext {
    pub app_state: AppState,
    pub save_index: SaveIndexFn,
}

pub type SaveIndexFn = Box<dyn Fn(&Index) -> Result<(), String> + Send + Sync + 'static>;

impl GraphQLContext {
    pub fn new(app_state: AppState, save_index: SaveIndexFn) -> Self {
        Self {
            app_state,
            save_index,
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
macro_rules! graphql_ctx_member {
    ($ctx_var: ident, $($member: ident).+, $mode: ident) => {{
        let ctx = $ctx_var.data::<$crate::graphql::GraphQLContext>().unwrap();
        ctx.$($member).+.$mode().await
    }};
}
