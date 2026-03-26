use std::{cmp::Ordering, num::NonZero};

use regex::Regex;

use crate::{
    index::{Album, Artist, CmpIndex, IndexCache, Rating, Track},
    manager::Ratings,
};

use super::pagination::{Paginated, Pagination};

pub fn search_tracks(
    query: &str,
    pagination: Pagination,
    index: &IndexCache,
    ratings: &Ratings,
) -> Paginated<Track> {
    let cmp_index = CmpIndex::new(index);

    search(
        query,
        pagination,
        index.tracks.iter(),
        |track, words| {
            let album = index.albums.get(&track.tags.album_id).unwrap();

            let mut score = 0;

            for word in words {
                score += contains_with_multiplier(&track.tags.title, word) * 5;
                score += contains_with_multiplier(&album.name, word) * 3;
            }

            if let Some(rating) = ratings.get(&track.id) {
                score = match rating {
                    Rating::Zero => 0,
                    Rating::One => score / 4,
                    Rating::Two => score / 2,
                    Rating::Three => score,
                    Rating::Four => score * 2,
                    Rating::Five => score * 3,
                };
            }

            score
        },
        |a, b| cmp_index.cmp_tracks(a, b),
    )
}

pub fn search_albums(query: &str, pagination: Pagination, index: &IndexCache) -> Paginated<Album> {
    let cmp_index = CmpIndex::new(index);

    search(
        query,
        pagination,
        index.albums.iter(),
        |album, words| {
            let mut score = 0;

            for word in words {
                score += contains_with_multiplier(&album.name, word) * 5;
            }

            score
        },
        |a, b| cmp_index.cmp_albums(a, b),
    )
}

pub fn search_artists(
    query: &str,
    pagination: Pagination,
    index: &IndexCache,
) -> Paginated<Artist> {
    search(
        query,
        pagination,
        index.artists.iter(),
        |artist, words| {
            let mut score = 0;

            for word in words {
                score += contains_with_multiplier(&artist.name, word) * 5;
            }

            score
        },
        CmpIndex::cmp_artists,
    )
}

fn search<'a, K: Copy + Eq + 'a, V: Clone + 'a>(
    query: &str,
    pagination: Pagination,
    items: impl Iterator<Item = (&'a K, &'a V)>,
    compute_score: impl Fn(&V, &[Regex]) -> usize,
    cmp_results: impl Fn(&V, &V) -> Ordering,
) -> Paginated<V> {
    let words: Vec<_> = query
        .split_whitespace()
        .map(str::trim)
        .filter(|word| !word.is_empty())
        .map(|word| Regex::new(&format!("(?i){}", regex::escape(word))).unwrap())
        .collect();

    let mut items = items
        .filter_map(|(k, v)| {
            NonZero::new(compute_score(v, &words))
                // Hide search results that have too low of a relevance
                .filter(|score| score.get() >= 5)
                .map(|score| (k, v, score))
        })
        .collect::<Vec<(&K, &V, NonZero<usize>)>>();

    items.sort_by(|(_, a_val, a_score), (_, b_val, b_score)| {
        a_score
            .cmp(b_score)
            .reverse()
            .then_with(|| cmp_results(a_val, b_val))
            .then_with(|| panic!("Unsorted values in search results"))
    });

    Paginated::paginate(items.into_iter(), pagination).map(|(_, v, _)| (*v).clone())
}

fn contains_with_multiplier(input: &str, word: &Regex) -> usize {
    word.find(input)
        .map_or(0, |pos| pos.len() * if pos.start() == 0 { 2 } else { 1 })
}
