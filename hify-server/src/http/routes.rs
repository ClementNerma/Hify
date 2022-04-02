use std::path::Path;

use rocket::{
    http::{ContentType, Status},
    response::content::Custom,
    tokio::fs::File,
    State,
};
use rocket_seek_stream::SeekStream;

use crate::index::{AlbumID, AudioFormat, TrackID};

use super::{
    server::{rest_server_error, FaillibleResponse},
    AppState,
};

#[rocket::get("/art/<id>")]
pub async fn art(ctx: &State<AppState>, id: String) -> FaillibleResponse<Custom<File>> {
    let index = ctx.index.read().await;
    let album_art_path = index
        .albums_arts
        .get(&AlbumID(id))
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

    Ok(Custom(mime_type, file))
}

#[rocket::get("/stream/<id>")]
pub async fn stream<'a>(
    ctx: &State<AppState>,
    id: String,
) -> FaillibleResponse<Custom<SeekStream<'a>>> {
    let index = ctx.index.read().await;
    let track_path = index
        .cache
        .tracks_paths
        .get(&TrackID(id.clone()))
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Provided track ID was not found".to_string(),
            )
        })?;

    let track = index.tracks.get(&TrackID(id)).unwrap();

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

    Ok(Custom(mime_type, stream))
}
