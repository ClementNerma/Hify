use juniper::{EmptySubscription, RootNode};

use super::{mutations::MutationRoot, queries::QueryRoot, state::GraphQLContext};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

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

pub fn get_graphql_schema() -> Schema {
    Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<GraphQLContext>::new(),
    )
}
