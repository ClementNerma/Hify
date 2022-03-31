use juniper::graphql_object;

use super::{entrypoint::OkScalar, GraphQLContext};
use crate::builder::build_index;

pub struct MutationRoot;

#[graphql_object(context = GraphQLContext)]

impl MutationRoot {
    async fn generate_index(ctx: &mut GraphQLContext) -> OkScalar {
        let index = build_index(ctx.index.read().await.from.clone());
        *ctx.index.write().await = index;
        OkScalar
    }
}