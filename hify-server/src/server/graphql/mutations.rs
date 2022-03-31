use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::builder::build_index;

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn generate_index(&self, ctx: &Context<'_>) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let index = build_index(ctx.index.read().await.from.clone());
        *ctx.index.write().await = index;
        true
    }
}
