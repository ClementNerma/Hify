use crate::http::AppState;

pub struct GraphQLContext {
    pub app_state: AppState,
}

impl GraphQLContext {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

#[macro_export]
macro_rules! graphql_index {
    ($ctx_var: ident) => {
        graphql_ctx_member!($ctx_var, index, read)
    };
}

#[macro_export]
macro_rules! graphql_ctx_member {
    ($ctx_var: ident, $member: ident, $mode: ident) => {{
        let ctx = $ctx_var.data::<GraphQLContext>().unwrap();
        ctx.app_state.$member.$mode().await
    }};
}
