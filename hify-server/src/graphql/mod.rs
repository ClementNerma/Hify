mod entrypoint;
mod mutations;
mod pagination;
mod queries;
mod routes;
mod state;

pub use entrypoint::{get_graphql_schema, AppSchema};
pub use pagination::Paginable;
pub use routes::get_graphql_routes;
pub use state::{GraphQLContext, SaveIndexFn};

#[macro_export]
macro_rules! define_scalar_string {
    ($typename: ident) => {
        #[::async_graphql::Scalar]
        impl ::async_graphql::ScalarType for $typename {
            fn parse(value: ::async_graphql::Value) -> ::async_graphql::InputValueResult<Self> {
                if let ::async_graphql::Value::String(value) = value {
                    Ok(Self(value))
                } else {
                    Err(::async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> ::async_graphql::Value {
                ::async_graphql::Value::String(self.0.to_string())
            }
        }
    };

    ($($typename: ident),+) => {
        $($crate::define_scalar_string!($typename);)+
    }
}
