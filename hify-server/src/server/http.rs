use rocket::Rocket;

use super::{cors::CORS, graphql::*};
use crate::index::Index;

pub async fn launch(index: Index) -> Result<(), rocket::Error> {
    Rocket::build()
        .attach(CORS)
        .manage(GraphQLContext::new(index))
        .manage(get_graphql_schema())
        .mount("/graphql", get_graphql_routes())
        .launch()
        .await
}
