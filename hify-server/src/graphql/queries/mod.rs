use crate::declare_gql_connection;

mod global;
mod on_types;

declare_gql_connection!(
    TrackIDConnection => TrackIDEdge,
    TrackUsizeConnection => TrackUsizeEdge,
    AlbumUsizeConnection => AlbumUsizeEdge
);

pub use self::global::QueryRoot;
