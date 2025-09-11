mod entrypoint;
mod mutations;
mod pagination;
mod queries;
mod state;

pub use self::{
    entrypoint::{AppSchema, get_graphql_schema},
    pagination::Paginable,
    state::{GraphQLContext, SaveIndexFn},
};
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

#[macro_export]
macro_rules! define_scalar_string {
    ($typename: ident) => {
        #[async_graphql::Scalar(name = "String")]
        impl async_graphql::ScalarType for $typename {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                if let async_graphql::Value::String(string) = &value
                    && let Ok(id) = <Self as $crate::index::IdType>::decode(string)
                {
                    Ok(id)
                } else {
                    Err(async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(<Self as $crate::index::IdType>::encode(self))
            }
        }
    };
}

/// NOTE: This scalar resolves to a `true` boolean (input & output)
///
/// We can't make it resolve to a `null` value as this would clash with GraphQL's spec
/// when using it as an input object for @oneOf()
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EmptyScalar;

#[Scalar(name = "Empty")]
impl ScalarType for EmptyScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::Boolean(true) => Ok(Self),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::Boolean(true)
    }
}
