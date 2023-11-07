mod entrypoint;
mod mutations;
mod pagination;
mod queries;
mod queries_types;
mod state;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
pub use entrypoint::{get_graphql_schema, AppSchema};
pub use pagination::Paginable;
pub use state::{GraphQLContext, SaveIndexFn};

#[macro_export]
macro_rules! define_scalar_string {
    ($typename: ident) => {
        #[async_graphql::Scalar(name = "String")]
        impl async_graphql::ScalarType for $typename {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                match value {
                    async_graphql::Value::String(value) => Ok(Self::decode(&value)?),
                    _ => Err(async_graphql::InputValueError::expected_type(value)),
                }
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(self.encode())
            }
        }
    };
}

type EmptyAnswer = &'static str;

const EMPTY_ANSWER: EmptyAnswer = "OK";

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EmptyScalar;

#[Scalar(name = "Empty")]
impl ScalarType for EmptyScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::Null => Ok(Self),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::Null
    }
}
