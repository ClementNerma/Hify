use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Field, Schema, STORED, STRING, TEXT},
    DocAddress, Document, Index as TantivyIndex, ReloadPolicy, Result, Score,
};

use super::{AlbumID, ArtistID, Index, TrackID};

pub static BASE_RAM_AMOUNT: usize = 3_000_000;
pub static RAM_AMOUNT_PER_TRACK: usize = 1_000;
pub static RESULTS_LIMIT: usize = 100;

pub struct SearchIndex {
    tracks: TantivyIndexAndSchema,
    albums: TantivyIndexAndSchema,
    artists: TantivyIndexAndSchema,
}

pub struct TantivyIndexAndSchema {
    index: TantivyIndex,
    id_field: Field,
    searchable_fields: Vec<Field>,
}

pub fn build_search_index(index: &Index) -> Result<SearchIndex> {
    Ok(SearchIndex {
        tracks: build_tracks_search_index(index)?,
        albums: build_albums_search_index(index)?,
        artists: build_artists_search_index(index)?,
    })
}

fn build_tracks_search_index(index: &Index) -> Result<TantivyIndexAndSchema> {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(TrackSearchField::TrackID.as_str(), STRING | STORED);
    schema_builder.add_text_field(TrackSearchField::Title.as_str(), TEXT);
    schema_builder.add_text_field(TrackSearchField::AlbumName.as_str(), TEXT);
    schema_builder.add_text_field(TrackSearchField::Artists.as_str(), TEXT);

    let schema = schema_builder.build();

    let t_index = TantivyIndex::create_in_ram(schema.clone());

    let id_field = schema
        .get_field(TrackSearchField::TrackID.as_str())
        .unwrap();
    let title_field = schema.get_field(TrackSearchField::Title.as_str()).unwrap();
    let album_name_field = schema
        .get_field(TrackSearchField::AlbumName.as_str())
        .unwrap();
    let artists_field = schema
        .get_field(TrackSearchField::Artists.as_str())
        .unwrap();

    let mut t_index_writer =
        t_index.writer(BASE_RAM_AMOUNT + index.tracks.len() * RAM_AMOUNT_PER_TRACK)?;

    for track in index.tracks.values() {
        let mut track_doc = Document::default();
        let tags = &track.metadata.tags;

        track_doc.add_text(id_field, track.id.clone().0);

        if let Some(ref title) = tags.title {
            track_doc.add_text(title_field, title.clone());
        }

        if let Some(ref album_name) = tags.album {
            track_doc.add_text(album_name_field, album_name.clone());
        }

        for artist in tags.get_artists_infos() {
            track_doc.add_text(artists_field, artist.name);
        }

        t_index_writer.add_document(track_doc)?;
    }

    t_index_writer.commit()?;

    Ok(TantivyIndexAndSchema {
        index: t_index,
        id_field,
        searchable_fields: vec![title_field, album_name_field, artists_field],
    })
}

pub fn build_albums_search_index(index: &Index) -> Result<TantivyIndexAndSchema> {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(AlbumSearchField::AlbumID.as_str(), STRING | STORED);
    schema_builder.add_text_field(AlbumSearchField::Name.as_str(), TEXT);
    schema_builder.add_text_field(AlbumSearchField::Artists.as_str(), TEXT);

    let schema = schema_builder.build();

    let t_index = TantivyIndex::create_in_ram(schema.clone());

    let id_field = schema
        .get_field(AlbumSearchField::AlbumID.as_str())
        .unwrap();
    let name_field = schema.get_field(AlbumSearchField::Name.as_str()).unwrap();
    let artists_field = schema
        .get_field(AlbumSearchField::Artists.as_str())
        .unwrap();

    let mut t_index_writer =
        t_index.writer(BASE_RAM_AMOUNT + index.tracks.len() * RAM_AMOUNT_PER_TRACK)?;

    for album in index.cache.albums_infos.values() {
        let mut album_doc = Document::default();

        album_doc.add_text(id_field, album.get_id().0);

        album_doc.add_text(name_field, album.name.clone());

        for artist in &album.album_artists {
            album_doc.add_text(artists_field, artist.name.clone());
        }

        t_index_writer.add_document(album_doc)?;
    }

    t_index_writer.commit()?;

    Ok(TantivyIndexAndSchema {
        index: t_index,
        id_field,
        searchable_fields: vec![name_field, artists_field],
    })
}

pub fn build_artists_search_index(index: &Index) -> Result<TantivyIndexAndSchema> {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(ArtistSearchField::ArtistID.as_str(), STRING | STORED);
    schema_builder.add_text_field(ArtistSearchField::Name.as_str(), TEXT);

    let schema = schema_builder.build();

    let t_index = TantivyIndex::create_in_ram(schema.clone());

    let id_field = schema
        .get_field(ArtistSearchField::ArtistID.as_str())
        .unwrap();
    let name_field = schema.get_field(ArtistSearchField::Name.as_str()).unwrap();

    let mut t_index_writer =
        t_index.writer(BASE_RAM_AMOUNT + index.tracks.len() * RAM_AMOUNT_PER_TRACK)?;

    for artist in index.cache.artists_infos.values() {
        let mut artist_doc = Document::default();

        artist_doc.add_text(id_field, artist.get_id().0);

        artist_doc.add_text(name_field, artist.name.clone());

        t_index_writer.add_document(artist_doc)?;
    }

    t_index_writer.commit()?;

    Ok(TantivyIndexAndSchema {
        index: t_index,
        id_field,
        searchable_fields: vec![name_field],
    })
}

pub fn search_inside_index(input: &str, search_index: &SearchIndex) -> Result<IndexSearchResults> {
    Ok(IndexSearchResults {
        tracks: search_one_index(input, &search_index.tracks, |id| TrackID(id.to_string()))?,
        albums: search_one_index(input, &search_index.albums, |id| AlbumID(id.to_string()))?,
        artists: search_one_index(input, &search_index.artists, |id| ArtistID(id.to_string()))?,
    })
}

fn search_one_index<Id>(
    input: &str,
    tias: &TantivyIndexAndSchema,
    id_mapper: impl Fn(&str) -> Id,
) -> Result<Vec<Id>> {
    let reader = tias
        .index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&tias.index, tias.searchable_fields.clone());

    let query = query_parser.parse_query(input)?;

    let top_docs: Vec<(Score, DocAddress)> =
        searcher.search(&query, &TopDocs::with_limit(RESULTS_LIMIT))?;

    let mut results = Vec::with_capacity(top_docs.len());

    for (_, doc_addr) in top_docs {
        let doc = searcher.doc(doc_addr)?;

        let id = doc
            .get_first(tias.id_field)
            .expect("Internal error: failed to get ID field in Tantivy document")
            .as_text()
            .expect("Internal error: ID field is not a string in Tantivy document");

        results.push(id_mapper(id));
    }

    Ok(results)
}

pub struct IndexSearchResults {
    pub tracks: Vec<TrackID>,
    pub albums: Vec<AlbumID>,
    pub artists: Vec<ArtistID>,
}

enum TrackSearchField {
    TrackID,
    Title,
    AlbumName,
    Artists,
}

impl TrackSearchField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::TrackID => "id",
            Self::Title => "title",
            Self::AlbumName => "album",
            Self::Artists => "artist",
        }
    }
}

enum AlbumSearchField {
    AlbumID,
    Name,
    Artists,
}

impl AlbumSearchField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::AlbumID => "id",
            Self::Name => "album",
            Self::Artists => "artist",
        }
    }
}

enum ArtistSearchField {
    ArtistID,
    Name,
}

impl ArtistSearchField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ArtistID => "id",
            Self::Name => "artist",
        }
    }
}
