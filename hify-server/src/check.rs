use crate::{
    index::{ArtTarget, Index},
    userdata::UserDataWrapper,
};

pub fn check_correctness(index: &Index, user_data: &UserDataWrapper) -> Result<(), Vec<String>> {
    let mut errors = vec![];

    macro_rules! error {
        ($message: tt, $($params: tt)*) => {{
            errors.push(format!($message, $($params)*));
        }}
    }

    let Index {
        from: _,
        fingerprint: _,
        tracks,
        cache: _, // TODO: check this as well
        arts,
    } = index;

    //
    // Check tracks ID
    //

    for (id, track) in tracks.iter() {
        if track.id != *id {
            error!(
                "Track registered with ID '{id:?}' in map but has ID '{:?}' instead",
                track.id
            );
        }
    }

    //
    // Check arts
    //

    for (id, art) in arts.iter() {
        if art.id != *id {
            error!(
                "Art registered with ID '{id:?}' in map but has ID '{:?}' instead",
                art.id
            );
        }

        match art.target {
            ArtTarget::AlbumCover(album_id) => {
                if !index.cache.albums_infos.contains_key(&album_id) {
                    error!(
                        "Art at path '{}' registered for unknown album ID '{album_id:?}'",
                        art.relative_path.display()
                    );
                }
            }

            ArtTarget::Artist(artist_id) => {
                if !index.cache.artists_infos.contains_key(&artist_id) {
                    error!(
                        "Art at path '{}' registered for unknown artist ID '{artist_id:?}'",
                        art.relative_path.display()
                    );
                }
            }
        }
    }

    //
    // Check overlaps in listening history
    //

    let history = user_data.history().entries();

    for (i, entry) in history.iter().enumerate().skip(1) {
        let against = history[i - 1].at + (time::Duration::SECOND * entry.duration_s);

        if entry.at < against && against - entry.at > time::Duration::SECOND {
            error!(
                "Overlapping entries in listening history ({}):\n* {:?}\n* {entry:?}",
                against - entry.at,
                history[i - 1],
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
