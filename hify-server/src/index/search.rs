use std::{collections::HashMap, time::Instant};

use async_graphql::SimpleObject;

use super::{AlbumInfos, ArtistInfos, Index, Track};

static SEARCH_CACHE_CAPACITY: usize = 100;
static SEARCH_CHARS_THRESOLD: usize = 3;

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

    if let Some(cached) = search_cache.content.get_mut(&words) {
        cached.last_usage = Instant::now();

        println!("|> Served cached search results.");
        return cached.results.clone();
    }

    let results = IndexSearchResults {
        tracks: search_and_score(index.tracks.values(), &words, limit),
        albums: search_and_score(index.cache.albums_infos.values(), &words, limit),
        artists: search_and_score(index.cache.artists_infos.values(), &words, limit),
    };

    if input.trim().len() >= SEARCH_CHARS_THRESOLD {
        if search_cache.content.len() == SEARCH_CACHE_CAPACITY {
            let min = search_cache.least_recently_used().unwrap().clone();
            search_cache.content.remove(&min);
        }

        search_cache.content.insert(
            words,
            SearchCacheEntry {
                last_usage: Instant::now(),
                results: results.clone(),
            },
        );

        let fill_percent = search_cache.content.len() as f64 * 100.0 / SEARCH_CACHE_CAPACITY as f64;

        println!(
            "|> Search cache now contains {} entries ({:.1}% of total capacity).",
            search_cache.content.len(),
            fill_percent
        );
    }

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

fn contains_with_multiplier(input: &str, word: &str) -> Option<usize> {
    let lower = input.to_lowercase();

    if !lower.contains(word) {
        None
    } else if lower.starts_with(word) {
        Some(2)
    } else {
        Some(1)
    }
}

trait SearchScoring {
    fn compute_word_scoring(&self, word: &str) -> usize;
}

impl SearchScoring for Track {
    fn compute_word_scoring(&self, word: &str) -> usize {
        let mut score = 0;

        let tags = &self.metadata.tags;

        if let Some(mul) = contains_with_multiplier(&tags.title, word) {
            score += word.len() * 10 * mul;
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

        if let Some(mul) = contains_with_multiplier(&self.name, word) {
            score += word.len() * 10 * mul;
        }

        for artist in &self.album_artists {
            if let Some(mul) = contains_with_multiplier(&artist.name, word) {
                score += word.len() * mul;
            }
        }

        score
    }
}

impl SearchScoring for ArtistInfos {
    fn compute_word_scoring(&self, word: &str) -> usize {
        if let Some(mul) = contains_with_multiplier(&self.name, word) {
            word.len() * 10 * mul
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

pub struct SearchCache {
    content: HashMap<Vec<String>, SearchCacheEntry>,
}

impl SearchCache {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

    pub fn least_recently_used(&self) -> Option<&Vec<String>> {
        self.content
            .iter()
            .fold(None, |min, (key, entry)| match min {
                None => Some((key, entry)),
                Some((_, min_entry)) if entry.last_usage < min_entry.last_usage => {
                    Some((key, entry))
                }
                Some(_) => min,
            })
            .map(|(min_key, _)| min_key)
    }
}

pub struct SearchCacheEntry {
    // search_terms: Vec<String>,
    // generated_at: Instant,
    last_usage: Instant,
    results: IndexSearchResults,
}
