use serde::{Deserialize, Serialize};

use crate::{
    index::{AlbumID, ArtistID, IdType, TrackID},
    os_struct,
};

use super::xml::XMLFieldValue;

pub const MUSIC_FOLDER_ID: u64 = 1;

os_struct! {
    pub struct AlbumID3WithSongs {
        id: AlbumID,
        name: String,
        version: Option<String>,
        #[rename = "artist"]
        artist_name: Option<String>,
        artist_id: Option<ArtistID>,
        #[rename = "cover_art"]
        cover_art_id: Option<CoverArtId>,
        song_count: usize,
        #[rename = "duration"]
        duration_s: u32,
        play_count: Option<usize>,
        #[rename = "created"]
        created_iso_8601: String,
        #[rename = "starred"]
        starred_iso_8601: Option<String>,
        year: Option<u16>,
        genre: Option<String>,
        #[rename = "played"]
        last_played_iso_8601: Option<String>,
        #[rename = "user_rating"]
        user_rating_1_to_5: Option<u8>,
        // TODO: record labels
        #[rename = "display_artist"]
        display_artist_name: Option<String>,
        // TODO: release_types
        // TODO: moods
        sort_name: Option<String>,
        is_compilation: Option<bool>,
        explicit_status: Option<String>,
        // TODO: disc_titles

        #[children] {
            original_release_date: Option<ItemDate>,
            release_date: Option<ItemDate>,
            genres: Option<Vec<ItemGenre>>,
            artists: Option<Vec<ArtistID3>>,
            #[rename = "song"]
            tracks: Option<Vec<Child>>
        }
    }
}

os_struct! {
    pub struct AlbumInfo {
        #[content_fields] {
            notes: Option<String>,
            music_brainz_id: Option<String>,
            last_fm_url: Option<String>,
            small_image_url: Option<String>,
            medium_image_url: Option<String>,
            large_image_url: Option<String>
        }
    }
}

os_struct! {
    pub struct ItemGenre {
        name: String,
    }
}

os_struct! {
    pub struct ArtistID3 {
        id: ArtistID,
        name: String,
        #[rename = "covertArt"]
        covert_art_id: Option<CoverArtId>,
        artist_image_url: Option<String>,
        album_count: Option<usize>,
        #[rename = "starred"]
        starred_iso_8601: Option<String>,
        music_brainz_id: Option<String>,
        sort_name: Option<String>,
        // roles: Option<Vec<String>>
    }
}

os_struct! {
    pub struct Artist {
        id: ArtistID,
        name: String,
        artist_image_url: Option<String>,
         #[rename = "starred"]
        starred_iso_8601: Option<String>,
        #[rename = "user_rating"]
        user_rating_1_to_5: Option<u8>,
        #[rename = "averageRating"]
        average_rating_1_to_5: Option<f32>,
    }
}

os_struct! {
    pub struct ArtistInfo2 {
         #[content_fields] {
            biography: Option<String>,
            music_brainz_id: Option<String>,
            last_fm_url: Option<String>,
            small_image_url: Option<String>,
            medium_image_url: Option<String>,
            large_image_url: Option<String>
        }

        #[children] {
            similar_artists: Option<Vec<ArtistID3>>
        }
    }
}

os_struct! {
    pub struct ItemDate {
        year: Option<u16>,
        month: Option<u8>,
        day: Option<u8>,
    }
}

os_struct! {
    pub struct Child {
        id: String,
        parent: Option<String>, // TODO: ChildId?
        is_dir: bool,
        title: String,
        #[rename = "album"]
        album_name: Option<String>,
        #[rename = "artist"]
        artist_name: Option<String>,
        #[rename = "track"]
        track_number: Option<u32>,
        year: Option<u32>,
        genre: Option<String>,
        #[rename = "covertArt"]
        covert_art_id: Option<CoverArtId>,
        #[rename = "size"]
        size_bytes: Option<u64>,
        #[rename = "contentType"]
        mime_type: Option<String>,
        #[rename = "suffix"]
        file_extension: Option<String>,
        // TODO: transcodedContentType
        // TODO: transcodedSuffix
        #[rename = "duration"]
        duration_s: Option<u32>,
        bit_rate: Option<u32>,
        bit_depth: Option<u32>,
        sampling_rate: Option<u32>,
        channel_count: Option<u8>,
        path: Option<String>,
        is_video: Option<bool>,
        #[rename = "userRating"]
        user_rating_1_to_5: Option<u8>,
        #[rename = "averageRating"]
        average_rating_1_to_5: Option<f32>,
        play_count: Option<usize>,
        disc_number: Option<u32>,
        #[rename = "created"]
        created_iso_8601: Option<String>,
        #[rename = "starred"]
        starred_iso_8601: Option<String>,
        album_id: Option<String>,
        artist_id: Option<String>,
        #[rename = "type"]
        typ: Option<&'static str>,
        // TODO: media_type
        // TODO: bookmark_position
        // TODO: original_width
        // TODO: original_height
        #[rename = "played"]
        last_played_iso_8601: Option<String>,
        bpm: Option<u16>,
        comment: Option<String>,
        sort_name: Option<String>,
        music_brainz_id: Option<String>,
        isrc: Option<String>,
        #[rename = "display_artist"]
        display_artist_name: Option<String>,
        #[rename = "display_album_artist"]
        display_album_artist_name: Option<String>,
        // replay_gain
        explicit_status: Option<String>,

        #[children] {
            genres: Option<Vec<ItemGenre>>,
            artists: Option<Vec<ArtistID3>>,
            album_artists: Option<Vec<ArtistID3>>,
            contributors: Option<Vec<Contributor>>
        }
    }
}

os_struct! {
    pub struct Contributor {
        role: String,
        sub_role: Option<String>,

        #[children] {
            artist: ArtistID3
        }
    }
}

os_struct! {
    pub struct MusicFolder {
        id: u64,
        name: &'static str,
    }
}

os_struct! {
    pub struct Genre {
        song_count: usize,
        album_count: usize,

        #[content]
        #[rename = "value"]
        name: String
    }
}

os_struct! {
    pub struct Bookmark {
        // TODO
    }
}

os_struct! {
    pub struct Playlist {
        // TODO
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CoverArtId {
    Track(TrackID),
    Album(AlbumID),
    Artist(ArtistID),
}

impl CoverArtId {
    pub fn encode(&self) -> String {
        match self {
            CoverArtId::Track(track_id) => {
                format!("track:{}", track_id.encode())
            }

            CoverArtId::Album(album_id) => {
                format!("album:{}", album_id.encode())
            }

            CoverArtId::Artist(artist_id) => {
                format!("artist:{}", artist_id.encode())
            }
        }
    }

    pub fn decode(str: &str) -> Result<Self, ()> {
        // Try decoding as a track
        str.strip_prefix("track:")
            .and_then(|str| TrackID::decode(str).ok())
            .map(Self::Track)
            // Then as an album
            .or_else(|| {
                str.strip_prefix("album:")
                    .and_then(|str| AlbumID::decode(str).ok())
                    .map(Self::Album)
            })
            // Then as an artist
            .or_else(|| {
                str.strip_prefix("artist:")
                    .and_then(|str| ArtistID::decode(str).ok())
                    .map(Self::Artist)
            })
            // All cases failed
            .ok_or(())
    }
}

impl From<CoverArtId> for XMLFieldValue {
    fn from(value: CoverArtId) -> Self {
        XMLFieldValue::String(value.encode())
    }
}
