use async_graphql::{EmptySubscription, Schema};

use crate::index::Index;

use super::{mutations::MutationRoot, queries::QueryRoot, GraphQLContext};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_graphql_schema(index: Index) -> AppSchema {
    AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(GraphQLContext::new(index))
        .finish()
}
