use std::path::Path;

use rocket::{
    http::{ContentType, Status},
    tokio::fs::File,
    State,
};
use rocket_seek_stream::SeekStream;

use crate::index::{AlbumID, ArtistID, AudioFormat, TrackID};

use super::{
    server::{rest_server_error, FaillibleResponse},
    AppState,
};

#[rocket::get("/art/album/<id>")]
pub async fn album_art(
    ctx: &State<AppState>,
    id: String,
) -> FaillibleResponse<(ContentType, File)> {
    let id = AlbumID::decode(&id)
        .map_err(|_| rest_server_error(Status::BadRequest, "Invalid ID provided".to_string()))?;

    let index = ctx.index.read().await;
    let album_art_path = index
        .albums_arts
        .get(&id)
        .cloned()
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Provided album ID was not found".to_string(),
            )
        })?
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Provided album does not have an art image".to_string(),
            )
        })?;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = album_art_path.extension().unwrap().to_str().unwrap();

    let mime_type = ContentType::from_extension(ext).ok_or_else(|| {
        rest_server_error(
            Status::InternalServerError,
            "Internal error: Rocket did not return a valid MIME-TYPE for an art file extension"
                .to_string(),
        )
    })?;

    let file = File::open(Path::new(&album_art_path))
        .await
        .map_err(|err| {
            rest_server_error(
                Status::InternalServerError,
                format!("Failed to open art file: {err}"),
            )
        })?;

    Ok((mime_type, file))
}

#[rocket::get("/art/artist/<id>")]
pub async fn artist_art(
    ctx: &State<AppState>,
    id: String,
) -> FaillibleResponse<(ContentType, File)> {
    let id = ArtistID::decode(&id)
        .map_err(|_| rest_server_error(Status::BadRequest, "Invalid ID provided".to_string()))?;

    let index = ctx.index.read().await;

    let artist_first_album_id = index
        .cache
        .artists_albums
        .get(&id)
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Provided artist ID was not found".to_string(),
            )
        })?
        .keys()
        .next()
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Provided artist does not have any album to generate an art image from".to_string(),
            )
        })?;

    let album_art_path = index
        .albums_arts
        .get(artist_first_album_id)
        .cloned()
        .expect("Internal error: album not found from provided artist's first album ID")
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Artist's first album does not have an art image".to_string(),
            )
        })?;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = album_art_path.extension().unwrap().to_str().unwrap();

    let mime_type = ContentType::from_extension(ext).ok_or_else(|| {
        rest_server_error(
            Status::InternalServerError,
            "Internal error: Rocket did not return a valid MIME-TYPE for an art file extension"
                .to_string(),
        )
    })?;

    let file = File::open(Path::new(&album_art_path))
        .await
        .map_err(|err| {
            rest_server_error(
                Status::InternalServerError,
                format!("Failed to open art file: {err}"),
            )
        })?;

    Ok((mime_type, file))
}

#[rocket::get("/stream/<id>")]
pub async fn stream<'a>(
    ctx: &State<AppState>,
    id: String,
) -> FaillibleResponse<(ContentType, SeekStream<'a>)> {
    let id = TrackID::decode(&id)
        .map_err(|_| rest_server_error(Status::BadRequest, "Invalid ID provided".to_string()))?;

    let index = ctx.index.read().await;
    let track_path = index.cache.tracks_paths.get(&id).ok_or_else(|| {
        rest_server_error(
            Status::NotFound,
            "Provided track ID was not found".to_string(),
        )
    })?;

    let track = index.tracks.get(&id).unwrap();

    let stream = SeekStream::from_path(&track_path).map_err(|err| {
        rest_server_error(
            Status::InternalServerError,
            format!("Failed to open seek stream for track file: {}", err),
        )
    })?;

    let mime_type = match track.metadata.format {
        AudioFormat::MP3 => ContentType::MPEG,
        AudioFormat::FLAC => ContentType::FLAC,
        AudioFormat::WAV => ContentType::WAV,
        AudioFormat::AAC => ContentType::AAC,
        AudioFormat::OGG => ContentType::OGG,
        AudioFormat::M4A => ContentType::MP4,
    };

    Ok((mime_type, stream))
}

#[rocket::get("/exit")]
pub fn exit() {
    std::process::exit(0)
}
