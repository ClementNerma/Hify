mod albums;
mod artists;
mod generate;
mod manager;
mod tools;

pub use self::{generate::*, manager::*};

pub static LARGE_ART_SIDE_PX: u32 = 2000;
pub static MEDIUM_ART_SIDE_PX: u32 = 500;
pub static SMALL_ART_SIDE_PX: u32 = 200;
