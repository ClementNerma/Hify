use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{http::Status, response::content, Route, State};

use super::entrypoint::AppSchema;

#[rocket::get("/")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::options("/")]
pub async fn graphql_preflight_handler() -> Status {
    Status::NoContent
}

#[rocket::get("/?<query..>")]
async fn graphql_query(schema: &State<AppSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<AppSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

pub fn get_graphql_routes() -> Vec<Route> {
    rocket::routes![
        graphql_playground,
        graphql_preflight_handler,
        graphql_query,
        graphql_request
    ]
}
