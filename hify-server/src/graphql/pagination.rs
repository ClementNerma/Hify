use async_graphql::{
    connection::{
        Connection, ConnectionNameType, CursorType, DefaultConnectionName, DefaultEdgeName, Edge,
        EdgeNameType, EmptyFields, EnableNodesField,
    },
    InputObject, OutputType, Result,
};

/// Pagination input for GraphQL
/// Can do one of the following:
/// * Fetch the N first items by specifying only `first`
/// * Fetch the N first items after a given one by specifying only `after` and `first`
/// * Fetch the N last items before a given one by specifying only `before` and `last`
#[derive(InputObject)]
pub struct PaginationInput {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

/// Result of a computed pagination process
/// Can be returned directly from a GraphQL query
pub type Paginated<C, T, N = DefaultConnectionName, E = DefaultEdgeName> =
    Result<Connection<C, T, EmptyFields, EmptyFields, N, E, EnableNodesField>>;

/// Compute a paginated result from a paginable container and a [`PaginationInput`]
/// Requires an index cache to quickly avoid performing a full slice lookup
pub fn paginate<
    C: CursorType + Send + Sync,
    T: OutputType + Clone,
    N: ConnectionNameType,
    E: EdgeNameType,
>(
    pagination: PaginationInput,
    items: impl Paginable<By = C, Item = T>,
    item_cursor: impl Fn(&T) -> C,
) -> Paginated<C, T, N, E> {
    raw_paginate(
        pagination,
        items,
        |item, _| item_cursor(item),
        |item| item.clone(),
    )
}

/// Compute a paginated result from a list of mappable items and a [`PaginationInput`]
pub fn paginate_mapped_slice<T, U: OutputType, N: ConnectionNameType, E: EdgeNameType>(
    pagination: PaginationInput,
    items: &[T],
    mapper: impl Fn(&T) -> U,
) -> Paginated<usize, U, N, E> {
    raw_paginate(pagination, items, |_, i| i, |item| mapper(item))
}

/// Compute raw pagination
pub fn raw_paginate<
    C: CursorType + Send + Sync,
    T,
    U: OutputType,
    N: ConnectionNameType,
    E: EdgeNameType,
>(
    pagination: PaginationInput,
    items: impl Paginable<By = C, Item = T>,
    item_cursor: impl Fn(&T, usize) -> C,
    mapper: impl Fn(&T) -> U,
) -> Paginated<C, U, N, E> {
    // Determine the starting cursor, the number of elements to get, as well as the direction from the pagination input
    let (cursor, count, direction) = match (
        pagination.after,
        pagination.before,
        pagination.first,
        pagination.last,
    ) {
        (None, None, None, None) => return Err("Please provide pagination parameters".into()),
        (Some(_), Some(_), _, _) => {
            return Err("Cannot provide both 'after' and 'before' parameters at once".into())
        }
        (_, _, Some(_), Some(_)) => {
            return Err("Cannot provide both 'first' and 'last' parameters at once".into())
        }
        (None, Some(_), None, None) | (Some(_), None, None, None) => {
            return Err("Please provide a number of elements to get".into())
        }
        (None, Some(_), _, None) => {
            return Err(
                "Specifying a 'before' parameter requires the 'last' parameter as well".into(),
            )
        }
        (None, Some(c), None, Some(i)) => (Some(c), i, Direction::Before),
        (Some(_), None, None, _) => {
            return Err(
                "Specifying an 'after' parameter requires the 'first' parameter as well".into(),
            )
        }
        (Some(c), None, Some(i), None) => (Some(c), i, Direction::After),
        (None, None, Some(i), None) => (None, i, Direction::After),
        (None, None, None, Some(_)) => {
            return Err("Please provide a cursor to paginate from".into())
        }
    };

    // Ensure the count is valid
    let count = usize::try_from(count).map_err(|_| "Invalid count number provided")?;

    // Compute index of the first element to get from the index cache
    let index = match cursor {
        None => 0,
        Some(ref cursor) => {
            let cursor = C::decode_cursor(cursor)
                .map_err(|e| format!("Failed to decode provided cursor: {}", e))?;

            items
                .get_index(&cursor)
                .ok_or("Provided cursor was not found")?
        }
    };

    // Compute index of the first element to retrieve, considering the direction
    let start_at = match direction {
        Direction::After => index + usize::from(cursor.is_some()),
        Direction::Before => {
            if index >= count {
                index - count
            } else {
                0
            }
        }
    };

    // Create a Relay value
    let mut connection =
        Connection::<C, U, _, _, N, E>::new(start_at > 0, start_at + count < items.len());

    // Compute the paginated results' edges lazily
    let edges = items
        .ordered_values()
        .iter()
        .skip(start_at)
        .take(count)
        .enumerate()
        .map(|(i, item)| Edge::new(item_cursor(item, i), mapper(item)));

    // Produce the paginated results
    match direction {
        Direction::After => connection.edges.extend(edges),
        Direction::Before => connection.edges.extend(edges.rev()),
    }

    // Success!
    Ok(connection)
}

pub trait Paginable {
    type By;
    type Item;

    fn len(&self) -> usize;
    fn get_index(&self, cursor: &Self::By) -> Option<usize>;
    fn ordered_values(&self) -> &[Self::Item];
}

impl<T> Paginable for &[T] {
    type By = usize;
    type Item = T;

    fn len(&self) -> usize {
        (self as &[T]).len()
    }

    fn get_index(&self, cursor: &Self::By) -> Option<usize> {
        if *cursor >= self.len() {
            None
        } else {
            Some(*cursor)
        }
    }

    fn ordered_values(&self) -> &[Self::Item] {
        self
    }
}

enum Direction {
    Before,
    After,
}

/// Implements the [`CursorType`] trait for tuple structs containing a single [`String`] item
#[macro_export]
macro_rules! transparent_cursor_type {
    ($typename: ident) => {
        impl async_graphql::connection::CursorType for $typename {
            type Error = std::num::ParseIntError;

            fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
                Ok($typename(u64::decode_cursor(s)?))
            }

            fn encode_cursor(&self) -> String {
                self.0.encode_cursor()
            }
        }
    };

    ($typename: ident, $($typenames: ident),+) => {
        transparent_cursor_type!{$typename}
        $(transparent_cursor_type!{$typenames})+
    }
}

/// Create a [`ConnectionNameType`] to provide to paginate()
#[macro_export]
macro_rules! declare_gql_connection {
    ($name: ident => $edge_name: ident) => {
        pub struct $name;

        impl async_graphql::connection::ConnectionNameType for $name {
            fn type_name<T: async_graphql::OutputType>() -> String {
                stringify!($name).to_owned()
            }
        }

        pub struct $edge_name;

        impl async_graphql::connection::EdgeNameType for $edge_name {
            fn type_name<T: async_graphql::OutputType>() -> String {
                stringify!($edge_name).to_owned()
            }
        }
    };

    ($($name: ident => $edge_name: ident),+) => {
        $( $crate::declare_gql_connection!($name => $edge_name); )+
    }
}
