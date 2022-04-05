use std::{collections::HashMap, io::Write};

use async_graphql::SimpleObject;
use rayon::iter::{ParallelBridge, ParallelIterator};

use super::{AlbumInfos, ArtistInfos, Index, Track};

static SEARCH_CACHE_CAPACITY: usize = 100;

pub fn search_index(
    index: &Index,
    search_cache: &mut SearchCache,
    input: &str,
    limit: usize,
) -> IndexSearchResults {
    let words: Vec<_> = input
        .split_whitespace()
        .map(str::trim)
        .map(str::to_lowercase)
        .filter(|str| !str.is_empty())
        .collect();

    if let Some(cached) = search_cache.get(&words) {
        println!("|> Served cached search results.");
        std::io::stdout().flush().unwrap();

        return cached.clone();
    }

    let results = IndexSearchResults {
        tracks: search_and_score(index.tracks.values(), &words, limit),
        albums: search_and_score(index.cache.albums_infos.values(), &words, limit),
        artists: search_and_score(index.cache.artists_infos.values(), &words, limit),
    };

    if search_cache.len() == SEARCH_CACHE_CAPACITY {
        let key = search_cache.keys().next().unwrap().clone();
        search_cache.remove(&key);
    }

    search_cache.insert(words, results.clone());

    println!(
        "|> Search cache now contains {} entries.",
        search_cache.len()
    );
    std::io::stdout().flush().unwrap();

    results
}

fn search_and_score<'t, T: Clone + Send + Ord + SearchScoring + 't>(
    items: impl Iterator<Item = &'t T> + Send,
    words: &[String],
    limit: usize,
) -> Vec<T>
where
    &'t T: Send,
{
    let mut items: Vec<_> = items
        .par_bridge()
        .filter_map(|item| {
            let mut score = 0;

            for word in words {
                match item.compute_word_scoring(word) {
                    0 => return None,
                    word_score => score += word_score,
                }
            }

            Some(SearchResult {
                item: item.clone(),
                score,
            })
        })
        .collect();

    items.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then_with(|| a.item.cmp(&b.item))
    });

    items
        .into_iter()
        .map(|result| result.item)
        .take(limit)
        .collect()
}

trait SearchScoring {
    fn compute_word_scoring(&self, word: &str) -> usize;
}

impl SearchScoring for Track {
    fn compute_word_scoring(&self, word: &str) -> usize {
        let mut score = 0;

        let tags = &self.metadata.tags;

        if tags.title.to_lowercase().contains(word) {
            score += word.len() * 10;
        }

        let album_infos = tags.get_album_infos();

        if album_infos.name.to_lowercase().contains(word) {
            score += word.len() * 3;
        }

        for artist in album_infos.album_artists {
            if artist.name.to_lowercase().contains(word) {
                score += word.len();
            }
        }

        score
    }
}

impl SearchScoring for AlbumInfos {
    fn compute_word_scoring(&self, word: &str) -> usize {
        let mut score = 0;

        if self.name.to_lowercase().contains(word) {
            score += word.len() * 10;
        }

        for artist in &self.album_artists {
            if artist.name.to_lowercase().contains(word) {
                score += word.len();
            }
        }

        score
    }
}

impl SearchScoring for ArtistInfos {
    fn compute_word_scoring(&self, word: &str) -> usize {
        if self.name.to_lowercase().contains(word) {
            word.len() * 10
        } else {
            0
        }
    }
}

struct SearchResult<T> {
    item: T,
    score: usize,
}

#[derive(SimpleObject, Clone)]
pub struct IndexSearchResults {
    pub tracks: Vec<Track>,
    pub albums: Vec<AlbumInfos>,
    pub artists: Vec<ArtistInfos>,
}

pub type SearchCache = HashMap<Vec<String>, IndexSearchResults>;
