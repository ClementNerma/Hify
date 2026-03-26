mod albums;
mod artists;
mod genres;
mod manager;
mod tools;

pub use self::{
    albums::generate_album_arts,
    artists::generate_artists_art,
    genres::generate_genres_art,
    manager::{ArtSize, ArtsManager},
};

pub static LARGE_ART_SIDE_PX: u32 = 2000;
pub static MEDIUM_ART_SIDE_PX: u32 = 500;
pub static SMALL_ART_SIDE_PX: u32 = 250;
pub static TINY_ART_SIDE_PX: u32 = 125;
