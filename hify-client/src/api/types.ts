import { type } from 'arktype'

export const fileTime = type({
  nanos_since_epoch: 'number',
  secs_since_epoch: 'number',
})

export type FileTime = typeof fileTime.infer

export const fileTimes = type({
  ctime: fileTime.or('null'),
  mtime: fileTime,
})

export type FileTimes = typeof fileTimes.infer

export const trackAudioCodec = type.enumerated('FLAC', 'OPUS', 'VORBIS', 'MP3', 'AAC')

export type TrackAudioCodec = typeof trackAudioCodec.infer

export const trackMetadata = type({
  durationS: 'number',
  audioCodec: trackAudioCodec,
})

export type TrackMetadata = typeof trackMetadata.infer

export const trackDate = type({
  year: 'number',
  month: 'number | null',
  day: 'number | null',
})

export type TrackDate = typeof trackDate.infer

export const trackTags = type({
  title: 'string',
  artistsId: 'string[]',
  composersId: 'string[]',
  albumId: 'string',
  discNumber: 'number | null',
  trackNumber: 'number | null',
  genresId: 'string[]',
  date: trackDate.or('null'),
})

export type TrackTags = typeof trackTags.infer

export const track = type({
  id: 'string',
  relativePath: 'string',
  fileSizeBytes: 'number',
  fileTimes: fileTimes,
  metadata: trackMetadata,
  tags: trackTags,
})

export type Track = typeof track.infer

export const album = type({
  id: 'string',
  name: 'string',
  artistsId: 'string[]',
})

export type Album = typeof album.infer

export const artist = type({
  id: 'string',
  name: 'string',
})

export type Artist = typeof artist.infer

export const genre = type({
  id: 'string',
  name: 'string',
})

export type Genre = typeof genre.infer

export const rating = type.enumerated('One', 'Two', 'Three', 'Four', 'Five')

export type Rating = typeof rating.infer

//
// DTOs
//

export const artistCompleteInfos = type({
  artist: artist,
  albumsCount: 'number',
  tracksCount: 'number',
})

export type ArtistCompleteInfos = typeof artistCompleteInfos.infer

export const genreCompleteInfos = type({
  genre: genre,
  albumsCount: 'number',
  tracksCount: 'number',
})

export type GenreCompleteInfos = typeof genreCompleteInfos.infer

export const albumCompleteInfos = type({
  album: album,
  artists: artistCompleteInfos.array(),
  genres: genreCompleteInfos.array(),
  tracksCount: 'number',
})

export type AlbumCompleteInfos = typeof albumCompleteInfos.infer

//
// Dependant types
//

export const trackCompleteInfos = type({
  track: track,
  album: albumCompleteInfos,
  artists: artistCompleteInfos.array(),
  genres: genreCompleteInfos.array(),
  rating: rating.or('null'),
})

export type TrackCompleteInfos = typeof trackCompleteInfos.infer

//
// Pagination
//

export const paginationDir = type.enumerated('ASC', 'DESC')

export type PaginationDir = typeof paginationDir.infer

export const pagination = type({
  limit: 'number',
  offset: 'number | null',
  dir: paginationDir,
})

export type Pagination = typeof pagination.infer

// Define the generic Paginated validator
export const paginated = type('<T>', {
  results: 'T[]',
  hasMore: 'boolean',
  total: 'number',
})

export type Paginated<T> = {
  results: T[]
  hasMore: boolean
  total: number
}

//
// Queries
//

export const artistsSort = type.enumerated(
  'NAME',
  'ALBUMS_COUNT',
  'TRACKS_COUNT',
  'GREAT_TRACKS_COUNT',
)

export type ArtistsSort = typeof artistsSort.infer

export const albumsSort = type.enumerated(
  'NAME',
  'ADDED',
  'DATE',
  'TRACKS_COUNT',
  'DURATION',
  'UNRATED_FIRST',
  'RATED_TRACKS_COUNT',
  'BEST_TRACKS_COUNT',
)

export type AlbumsSort = typeof albumsSort.infer

export const tracksSort = type.enumerated('DATE', 'TITLE', 'DURATION', 'USER_RATING')

export type TracksSort = typeof tracksSort.infer

export const genresSort = type.enumerated(
  'NAME',
  'ALBUMS_COUNT',
  'TRACKS_COUNT',
  'GREAT_TRACKS_COUNT',
)

export type GenresSort = typeof genresSort.infer

export const artSize = type.enumerated('tiny', 'small', 'medium', 'large')

export type ArtSize = typeof artSize.infer

export const userMixSource = type.or(
  { type: '"all"' },
  { type: '"artist"', id: 'string' },
  { type: '"genre"', id: 'string' },
)

export type UserMixSource = typeof userMixSource.infer

export const userMixFilter = type.enumerated(
  'NOT_RATED',
  'NOT_BADLY_RATED',
  'WELL_RATED',
  'BEST_RATED',
  'INCLUDE_ALL',
)

export type UserMixFilter = typeof userMixFilter.infer

export const userMixParams = type({
  source: userMixSource,
  filter: userMixFilter,
  seed: 'number',
})

export type UserMixParams = typeof userMixParams.infer
