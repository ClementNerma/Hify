use juniper::{Context, EmptySubscription, RootNode};
use rocket::{response::content, tokio::sync::RwLock, Rocket, State};
use std::{path::PathBuf, sync::Arc};

use super::{mutations::MutationRoot, queries::QueryRoot};
use crate::index::Library;

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

pub struct GraphQLContext {
    pub root_path: PathBuf,
    pub index: Arc<RwLock<Option<Library>>>,
}

impl Context for GraphQLContext {}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
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

pub async fn launch(root_path: PathBuf) -> Result<(), rocket::Error> {
    Rocket::build()
        .manage(GraphQLContext {
            root_path,
            index: Arc::new(RwLock::new(None)),
        })
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
            EmptySubscription::<GraphQLContext>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
}
