use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::index::build_index;

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn generate_index(&self, ctx: &Context<'_>) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let index = build_index(ctx.app_state.index.read().await.from.clone());
        *ctx.app_state.index.write().await = index;
        true
    }
}
