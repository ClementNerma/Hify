mod entrypoint;
mod mutations;
mod pagination;
mod queries;
mod queries_types;
mod state;

pub use entrypoint::{get_graphql_schema, AppSchema};
pub use pagination::Paginable;
pub use state::{GraphQLContext, SaveIndexFn};

#[macro_export]
macro_rules! define_scalar_string {
    ($typename: ident) => {
        #[::async_graphql::Scalar(name = "String")]
        impl ::async_graphql::ScalarType for $typename {
            fn parse(value: ::async_graphql::Value) -> ::async_graphql::InputValueResult<Self> {
                if let ::async_graphql::Value::String(value) = value {
                    Ok(Self::decode(&value)?)
                } else {
                    Err(::async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> ::async_graphql::Value {
                ::async_graphql::Value::String(self.encode())
            }
        }
    };
}
