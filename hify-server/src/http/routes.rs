use rocket::{
    http::{ContentType, Status},
    tokio::fs::File,
    State,
};
use rocket_seek_stream::SeekStream;

use crate::index::{ArtID, ArtTarget, ArtistID, AudioFormat, TrackID};

use super::{
    server::{rest_server_error, FaillibleResponse},
    AppState,
};

#[rocket::get("/art/<id>")]
pub async fn art(ctx: &State<AppState>, id: String) -> FaillibleResponse<(ContentType, File)> {
    let id = ArtID::decode(&id).map_err(|_| {
        rest_server_error(Status::BadRequest, "Invalid art ID provided".to_string())
    })?;

    let index = ctx.index.read().await;
    let art = index.arts.get(&id).cloned().ok_or_else(|| {
        rest_server_error(Status::NotFound, "Provided art was not found".to_string())
    })?;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = art.relative_path.extension().unwrap().to_str().unwrap();

    let mime_type = ContentType::from_extension(ext).ok_or_else(|| {
        rest_server_error(
            Status::InternalServerError,
            "Internal error: Rocket did not return a valid MIME-TYPE for an art file extension"
                .to_string(),
        )
    })?;

    let file = File::open(index.from.join(&art.relative_path))
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

    let album_art = index
        .arts
        .get(&ArtTarget::AlbumCover(*artist_first_album_id).to_id())
        .ok_or_else(|| {
            rest_server_error(
                Status::NotFound,
                "Artist's first album does not have an art image".to_string(),
            )
        })?;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = album_art
        .relative_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    let mime_type = ContentType::from_extension(ext).ok_or_else(|| {
        rest_server_error(
            Status::InternalServerError,
            "Internal error: Rocket did not return a valid MIME-TYPE for an art file extension"
                .to_string(),
        )
    })?;

    let file = File::open(&album_art.relative_path).await.map_err(|err| {
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
    let track = index.tracks.get(&id).ok_or_else(|| {
        rest_server_error(
            Status::NotFound,
            "Provided track ID was not found".to_string(),
        )
    })?;

    let stream = SeekStream::from_path(index.from.join(&track.relative_path)).map_err(|err| {
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
        AudioFormat::OPUS => ContentType::OGG,
    };

    Ok((mime_type, stream))
}

#[rocket::get("/exit")]
pub fn exit() {
    std::process::exit(0)
}
