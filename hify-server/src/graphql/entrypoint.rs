use async_graphql::{EmptySubscription, Schema};

use crate::http::AppState;

use super::{mutations::MutationRoot, queries::QueryRoot, GraphQLContext};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_graphql_schema(app_state: AppState) -> AppSchema {
    AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(GraphQLContext::new(app_state))
        .finish()
}
