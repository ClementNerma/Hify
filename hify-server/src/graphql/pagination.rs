use std::hash::Hash;

use async_graphql::{
    connection::{Connection, CursorType, Edge},
    InputObject, Result,
};

use crate::index::SortedMap;

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
pub type Paginated<C, T> = Result<Connection<C, T>>;

/// Compute a paginated result from a list of items and a [`PaginationInput`]
/// Requires an index cache to quickly avoid performing a full slice lookup
pub fn paginate<C: CursorType + Eq + Hash, T: Clone + Ord>(
    pagination: PaginationInput,
    items: &SortedMap<C, T>,
    item_cursor: impl Fn(&T) -> C,
) -> Paginated<C, T> {
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
        Direction::After => index + if cursor.is_some() { 1 } else { 0 },
        Direction::Before => {
            if index >= count {
                index - count
            } else {
                0
            }
        }
    };

    // Create a Relay value
    let mut connection = Connection::<C, T>::new(start_at > 0, start_at + count < items.len());

    // Compute the paginated results' edges lazily
    let edges = items
        .values()
        .skip(start_at)
        .take(count)
        .map(|item| Edge::new(item_cursor(item), item.clone()));

    // Produce the paginated results
    match direction {
        Direction::After => connection.append(edges),
        Direction::Before => connection.append(edges.rev()),
    }

    // Success!
    Ok(connection)
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
            type Error = std::convert::Infallible;

            fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
                Ok($typename(String::decode_cursor(s)?))
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
