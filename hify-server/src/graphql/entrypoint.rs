use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::http::HttpState;

use super::{mutations::MutationRoot, queries::QueryRoot, state::SaveIndexFn, GraphQLContext};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_graphql_schema(app_state: Arc<HttpState>, save_index: SaveIndexFn) -> AppSchema {
    AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(GraphQLContext::new(app_state, save_index))
        .finish()
}
