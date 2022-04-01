use std::{collections::HashMap, hash::Hash};

use async_graphql::{
    connection::{Connection, CursorType, Edge},
    InputObject, Result,
};

#[derive(InputObject)]
pub struct PaginationInput {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

pub fn paginate<'a, C: CursorType + Eq + Hash, T: Clone>(
    pagination: PaginationInput,
    items: &[T],
    indexes_cache: &HashMap<C, usize>,
    item_cursor: impl Fn(&T) -> C,
) -> Result<Connection<C, T>> {
    // find item with cursor 'after' or 'before'
    // get nth items before or after

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

    let count = usize::try_from(count).map_err(|_| "Invalid count number provided")?;

    let cursor = cursor
        .map(|cursor| {
            C::decode_cursor(&cursor)
                .map_err(|e| format!("Failed to decode provided cursor: {}", e))
        })
        .transpose()?;

    let index = cursor
        .map(|cursor| {
            indexes_cache
                .get(&cursor)
                .ok_or("Provided cursor was not found")
        })
        .transpose()?
        .unwrap_or(&0);

    let start_at = match direction {
        Direction::After => *index,
        Direction::Before => {
            if *index >= count {
                *index - count
            } else {
                0
            }
        }
    };

    let mut connection = Connection::<C, T>::new(*index > 0, index + count < items.len());

    let edges = items
        .iter()
        .skip(start_at)
        .map(|item| Edge::new(item_cursor(item), item.clone()));

    match direction {
        Direction::After => connection.append(edges.take(count)),
        Direction::Before => connection.append(edges.rev().skip(1).take(count)),
    }

    Ok(connection)
}

enum Direction {
    Before,
    After,
}

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
}
