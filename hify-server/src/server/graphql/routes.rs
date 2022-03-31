use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::{http::Status, response::content, Route, State};

use super::{entrypoint::Schema, GraphQLContext};

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[rocket::options("/")]
pub async fn graphql_preflight_handler() -> Status {
    Status::NoContent
}

#[rocket::get("/?<request>")]
pub async fn get_graphql_handler(
    context: &State<GraphQLContext>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::post("/", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<GraphQLContext>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(&*schema, &*context).await
}

pub fn get_graphql_routes() -> Vec<Route> {
    rocket::routes![
        graphiql,
        graphql_preflight_handler,
        get_graphql_handler,
        post_graphql_handler
    ]
}
