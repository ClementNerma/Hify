use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::http::HttpState;

use super::{GraphQLContext, mutations::MutationRoot, queries::QueryRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_graphql_schema(app_state: Arc<HttpState>) -> AppSchema {
    AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(GraphQLContext::new(app_state))
        .finish()
}
