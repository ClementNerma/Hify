use std::{fs::File, path::Path};

use anyhow::{Context, Result, bail, ensure};
use symphonia::core::{
    codecs::{
        CODEC_TYPE_AAC, CODEC_TYPE_FLAC, CODEC_TYPE_MP3, CODEC_TYPE_NULL, CODEC_TYPE_OPUS,
        CODEC_TYPE_VORBIS, CODEC_TYPE_WAVPACK,
    },
    errors::Error,
    formats::{FormatOptions, FormatReader},
    io::{MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    probe::Hint,
    units::{Time, TimeBase},
};

use crate::index::{AudioCodec, TrackMetadata};

use super::adapter::convert_symphonia_metadata;

pub fn analyze_file(path: &Path) -> Result<TrackMetadata> {
    let src = File::open(path).context("Failed")?;

    let file_size = src
        .metadata()
        .context("Failed to get file's metadata")?
        .len();

    let mss = MediaSourceStream::new(Box::new(src), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    hint.with_extension(
        path.extension()
            .context("File does not have an extension")?
            .to_str()
            .context("File extension contains invalid UTF-8 characters")?,
    );

    let mut probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Found unsupported codec")?;

    let mut format = probed.format;

    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.
    let rev = format
        .metadata()
        .current()
        .cloned()
        .or_else(|| {
            probed
                .metadata
                .get()
                .as_ref()
                .and_then(|m| m.current())
                .cloned()
        })
        .context("No metadata was found in audio file")?;

    let mut tracks_iter = format
        .tracks()
        .iter()
        .filter(|t| t.codec_params.codec != CODEC_TYPE_NULL);

    let track = tracks_iter
        .next()
        .context("No supported audio track found")?;

    ensure!(
        tracks_iter.next().is_none(),
        "Multiple audio tracks found in file"
    );

    let codec = match track.codec_params.codec {
        CODEC_TYPE_MP3 => AudioCodec::MP3,
        CODEC_TYPE_FLAC => AudioCodec::FLAC,
        CODEC_TYPE_WAVPACK => AudioCodec::WAV,
        CODEC_TYPE_AAC => AudioCodec::AAC,
        CODEC_TYPE_VORBIS => AudioCodec::VORBIS,
        CODEC_TYPE_OPUS => AudioCodec::OPUS,
        _ => bail!("Found unknown codec: {}", track.codec_params.codec),
    };

    let time_base = track
        .codec_params
        .time_base
        .context("No time base found for audio file")?;

    let track_id = track.id;

    let dur = match track.codec_params.n_frames {
        Some(n_frames) => time_base.calc_time(n_frames),
        None => analyze_entire_file(format, track_id, time_base)?,
    };

    Ok(TrackMetadata {
        codec,
        file_size,
        duration: u32::try_from(dur.seconds)
            .context("Audio track is longer than 2^32-1 seconds!")?
            + if dur.frac > 0.5 { 1 } else { 0 },
        tags: convert_symphonia_metadata(rev)?,
    })
}

fn analyze_entire_file(
    mut reader: Box<dyn FormatReader>,
    track_id: u32,
    time_base: TimeBase,
) -> Result<Time, Error> {
    let mut raw_dur = 0;

    loop {
        // Read next packet
        let packet = match reader.next_packet() {
            Ok(packet) => packet,
            Err(err) => match err {
                Error::IoError(err)
                    if err.kind() == std::io::ErrorKind::UnexpectedEof
                        && err.to_string() == "end of stream" =>
                {
                    // Do not treat "end of stream" as a fatal error. It's the currently only way a
                    // format reader can indicate the media is complete.
                    break Ok(time_base.calc_time(raw_dur));
                }

                _ => break Err(err),
            },
        };

        // This should always be true since we previously ensured there was only one track in the audio file
        assert!(packet.track_id() == track_id);

        // Gapless playback is not enabled, so these values should be zero
        assert_eq!(packet.trim_start, 0);
        assert_eq!(packet.trim_end, 0);

        // Add packet duration to the total
        raw_dur += packet.dur
    }
}
