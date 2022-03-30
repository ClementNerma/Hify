use juniper::{Context, EmptySubscription, RootNode};
use rocket::{http::Status, response::content, tokio::sync::RwLock, Rocket, State};
use std::sync::Arc;

use super::{cors::CORS, mutations::MutationRoot, queries::QueryRoot};
use crate::index::Index;

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

pub struct GraphQLContext {
    pub index: Arc<RwLock<Index>>,
}

impl Context for GraphQLContext {}

pub struct OkScalar;

#[juniper::graphql_scalar(name = "OkScalar", description = "An Ok scalar")]
impl<S: ScalarValue> GraphQLScalar for OkScalar {
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(true)
    }

    fn from_input_value(value: &juniper::InputValue) -> Option<Self> {
        value.as_string_value().map(|_| OkScalar)
    }

    fn from_str<'a>(value: juniper::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[rocket::options("/graphql")]
async fn graphql_preflight_handler() -> Status {
    Status::NoContent
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<GraphQLContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<GraphQLContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

pub async fn launch(index: Index) -> Result<(), rocket::Error> {
    Rocket::build()
        .attach(CORS)
        .manage(GraphQLContext {
            index: Arc::new(RwLock::new(index)),
        })
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
            EmptySubscription::<GraphQLContext>::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                graphiql,
                graphql_preflight_handler,
                get_graphql_handler,
                post_graphql_handler
            ],
        )
        .launch()
        .await
}
