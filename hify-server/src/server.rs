use juniper::{graphql_object, Context, EmptySubscription, RootNode};
use rocket::{response::content, tokio::sync::RwLock, Rocket, State};
use std::{path::PathBuf, sync::Arc};

use crate::{builder::build_index, index::Library};

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

struct GraphQLContext {
    root_path: PathBuf,
    index: Arc<RwLock<Option<Library>>>,
}

struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index(ctx: &GraphQLContext, fingerprint: String) -> Option<Library> {
        ctx.index
            .read()
            .await
            .clone()
            .filter(|index| index.creation_time != fingerprint)
    }
}

struct MutationRoot;

#[graphql_object(context = GraphQLContext)]

impl MutationRoot {
    async fn generate_index(ctx: &mut GraphQLContext) -> Library {
        let index = build_index(&ctx.root_path);
        *ctx.index.write().await = Some(index.clone());
        index
    }
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
