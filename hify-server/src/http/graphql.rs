use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};

use crate::graphql::AppSchema;

use super::server::GRAPHQL_ENDPOINT;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint(GRAPHQL_ENDPOINT).finish())
}
