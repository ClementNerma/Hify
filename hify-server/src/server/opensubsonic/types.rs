use serde::{Deserialize, Serialize};

use crate::index::{AlbumID, ArtistID, IdType, TrackID};

pub const MUSIC_FOLDER_ID: u64 = 1;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3WithSongs {
    pub id: AlbumID,
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "artist", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<ArtistID>,
    #[serde(rename = "coverArt", skip_serializing_if = "Option::is_none")]
    pub cover_art_id: Option<CoverArtId>,
    pub song_count: usize,
    #[serde(rename = "duration")]
    pub duration_s: u32,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<usize>,
    #[serde(rename = "created")]
    pub created_iso_8601: String,
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred_iso_8601: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(rename = "played", skip_serializing_if = "Option::is_none")]
    pub last_played_iso_8601: Option<String>,
    #[serde(rename = "userRating", skip_serializing_if = "Option::is_none")]
    pub user_rating_1_to_5: Option<u8>,
    #[serde(rename = "displayArtist", skip_serializing_if = "Option::is_none")]
    pub display_artist_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compilation: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_status: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_release_date: Option<ItemDate>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<ItemDate>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<ItemGenre>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistID3>>,
    #[serde(rename = "song")]
    pub tracks: Vec<Child>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumInfo {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fm_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
}

#[derive(Serialize)]
pub struct ItemGenre {
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistID3 {
    pub id: ArtistID,
    pub name: String,
    #[serde(rename = "coverArt", skip_serializing_if = "Option::is_none")]
    pub covert_art_id: Option<CoverArtId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_image_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_count: Option<usize>,
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred_iso_8601: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct Artist {
    pub id: ArtistID,
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_image_url: Option<String>,
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred_iso_8601: Option<String>,
    #[serde(rename = "userRating", skip_serializing_if = "Option::is_none")]
    pub user_rating_1_to_5: Option<u8>,
    #[serde(rename = "averageRating", skip_serializing_if = "Option::is_none")]
    pub average_rating_1_to_5: Option<f32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistInfo2 {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biography: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fm_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
    
    #[serde(rename = "similarArtist", skip_serializing_if = "Option::is_none")]
    pub similar_artists: Option<Vec<ArtistID3>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDate {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub id: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    pub is_dir: bool,
    pub title: String,
    #[serde(rename = "album", skip_serializing_if = "Option::is_none")]
    pub album_name: Option<String>,
    #[serde(rename = "artist", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<String>,
    #[serde(rename = "track", skip_serializing_if = "Option::is_none")]
    pub track_number: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(rename = "coverArt", skip_serializing_if = "Option::is_none")]
    pub covert_art_id: Option<CoverArtId>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration_s: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_count: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_video: Option<bool>,
    #[serde(rename = "userRating", skip_serializing_if = "Option::is_none")]
    pub user_rating_1_to_5: Option<u8>,
    #[serde(rename = "averageRating", skip_serializing_if = "Option::is_none")]
    pub average_rating_1_to_5: Option<f32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<u32>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created_iso_8601: Option<String>,
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred_iso_8601: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub typ: Option<&'static str>,
    #[serde(rename = "played", skip_serializing_if = "Option::is_none")]
    pub last_played_iso_8601: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpm: Option<u16>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<Vec<String>>,
    #[serde(rename = "displayArtist", skip_serializing_if = "Option::is_none")]
    pub display_artist_name: Option<String>,
    #[serde(rename = "displayAlbumArtist", skip_serializing_if = "Option::is_none")]
    pub display_album_artist_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_status: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<ItemGenre>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistID3>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_artists: Option<Vec<ArtistID3>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    pub role: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_role: Option<String>,
    pub artist: ArtistID3,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: u64,
    pub name: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub song_count: usize,
    pub album_count: usize,
    #[serde(rename = "value")]
    pub name: String,
}

#[derive(Serialize)]
pub struct Bookmark {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    pub id: String,
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "songCount")]
    pub song_count: usize,
    #[serde(rename = "duration")]
    pub duration_s: u32,
    #[serde(rename = "created")]
    pub created_iso_8601: String,
    #[serde(rename = "changed")]
    pub changed_iso_8601: String,
    #[serde(rename = "coverArt", skip_serializing_if = "Option::is_none")]
    pub cover_art_id: Option<CoverArtId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until_iso_8601: Option<String>,
    #[serde(rename = "entry")]
    pub tracks: Vec<Child>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueue {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<TrackID>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    pub username: String,
    #[serde(rename = "changed")]
    pub changed_iso_8601: String,
    #[serde(rename = "changedBy")]
    pub changed_by_app: String,
    #[serde(rename = "entry", skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<Child>>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(untagged)]
pub enum CoverArtId {
    Track(TrackID),
    Album(AlbumID),
    Artist(ArtistID),
}

impl CoverArtId {
    pub fn encode(&self) -> String {
        match self {
            Self::Track(track_id) => {
                format!("track:{}", track_id.encode())
            }

            Self::Album(album_id) => {
                format!("album:{}", album_id.encode())
            }

            Self::Artist(artist_id) => {
                format!("artist:{}", artist_id.encode())
            }
        }
    }

    pub fn decode(str: &str) -> Result<Self, ()> {
        str.strip_prefix("track:")
            .and_then(|str| TrackID::decode(str).ok())
            .map(Self::Track)
            .or_else(|| {
                str.strip_prefix("album:")
                    .and_then(|str| AlbumID::decode(str).ok())
                    .map(Self::Album)
            })
            .or_else(|| {
                str.strip_prefix("artist:")
                    .and_then(|str| ArtistID::decode(str).ok())
                    .map(Self::Artist)
            })
            .ok_or(())
    }
}

impl Serialize for CoverArtId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.encode())
    }
}
