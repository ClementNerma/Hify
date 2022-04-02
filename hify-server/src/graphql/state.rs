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
    ($ctx_var: ident) => {{
        let ctx = $ctx_var.data::<GraphQLContext>().unwrap();
        ctx.app_state.index.read().await
    }};
}
