use juniper::graphql_object;

use super::graphql::{GraphQLContext, OkScalar};
use crate::builder::build_index;

pub struct MutationRoot;

#[graphql_object(context = GraphQLContext)]

impl MutationRoot {
    async fn generate_index(ctx: &mut GraphQLContext) -> OkScalar {
        let index = build_index(&ctx.root_path);
        *ctx.index.write().await = Some(index);
        OkScalar
    }
}
