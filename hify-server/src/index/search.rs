use std::{collections::HashMap, time::Instant};

use async_graphql::SimpleObject;
use log::debug;

use super::{AlbumInfos, ArtistInfos, Index, Rating, Track};

static SEARCH_CACHE_CAPACITY: usize = 100;
static SEARCH_CHARS_THRESOLD: usize = 3;

pub struct SearchOptions<'a, 'b> {
    pub search_cache: Option<&'a mut SearchCache>,
    pub tracks_user_score: Option<AltScoreFn<'b, Track>>,
}

pub type AltScoreFn<'a, T> = &'a dyn Fn(&T) -> Option<Rating>;

pub fn search_index(
    index: &Index,
    mut opts: SearchOptions,
    input: &str,
    limit: usize,
) -> IndexSearchResults {
    let words: Vec<_> = input
        .split_whitespace()
        .map(str::trim)
        .map(str::to_lowercase)
        .filter(|str| !str.is_empty())
        .collect();

    if let Some(cached) = opts
        .search_cache
        .as_mut()
        .and_then(|cache| cache.content.get_mut(&words))
    {
        cached.last_usage = Instant::now();

        debug!("|> Served cached search results.");
        return cached.results.clone();
    }

    let results = IndexSearchResults {
        tracks: search_and_score(index.tracks.values(), &words, limit, opts.tracks_user_score),
        albums: search_and_score(
            index.cache.albums_infos.values(),
            &words,
            limit,
            // TODO
            None,
        ),
        artists: search_and_score(
            index.cache.artists_infos.values(),
            &words,
            limit,
            // TODO
            None,
        ),
    };

    if let Some(search_cache) = opts.search_cache.as_mut()
        && input.trim().len() >= SEARCH_CHARS_THRESOLD
    {
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

        debug!(
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
    alt_score: Option<AltScoreFn<T>>,
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

            if let Some(rating) = alt_score.and_then(|alt_score| alt_score(item)) {
                score = match rating.value() {
                    0..=4 => score / 2,
                    5..=7 => score,
                    8..=9 => score * 2,
                    10 => score * 3,
                    _ => unreachable!(),
                };
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
    } else if lower.split_whitespace().next() == Some(word) {
        Some(20)
    } else if lower.starts_with(word) {
        Some(15)
    } else {
        Some(10)
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
            score += word.len() * mul;
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

    pub fn clear(&mut self) {
        self.content.clear();
    }
}

pub struct SearchCacheEntry {
    // search_terms: Vec<String>,
    // generated_at: Instant,
    last_usage: Instant,
    results: IndexSearchResults,
}
