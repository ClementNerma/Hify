use async_graphql::Result;

pub trait GraphQLInto<T>: Sized + TryInto<T> {
    fn graphql_into(self, field_name: &'static str) -> Result<T> {
        self.try_into()
            .map_err(|_| format!("Invalid GraphQL value provided for field '{field_name}'").into())
    }
}

impl<F: Sized + TryInto<T>, T> GraphQLInto<T> for F {}

#[macro_export]
macro_rules! graphql_into {
    ($name: ident) => {
        $name.graphql_into(stringify!($name))?
    };
}
