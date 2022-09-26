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
        graphql_ctx_member!($ctx_var, index, read)
    };
}

#[macro_export]
macro_rules! graphql_user_data {
    ($ctx_var: ident) => {
        graphql_ctx_member!($ctx_var, user_data, read)
    };
}

#[macro_export]
macro_rules! graphql_ctx_member {
    ($ctx_var: ident, $member: ident, $mode: ident) => {{
        let ctx = $ctx_var.data::<GraphQLContext>().unwrap();
        ctx.app_state.$member.$mode().await
    }};
}
