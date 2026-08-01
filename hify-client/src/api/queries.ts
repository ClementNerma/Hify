import { queryApi, queryApiKeyed } from './fetch'
import type { CachableQuery } from './hooks'
import {
  albumCompleteInfos,
  artistCompleteInfos,
  trackCompleteInfos,
  paginated,
  type AlbumsSort,
  type ArtistsSort,
  type AlbumCompleteInfos,
  type TrackCompleteInfos,
  type GenresSort,
  type Paginated,
  type Pagination,
  type TracksSort,
  type ArtistCompleteInfos,
  type GenreCompleteInfos,
  genreCompleteInfos,
  type UserMixParams,
} from './types'

export function fetchAlbums(
  params: Pagination & { sortBy: AlbumsSort },
): CachableQuery<Paginated<AlbumCompleteInfos>> {
  return queryApiKeyed('/albums', params, paginated(albumCompleteInfos))
}

export function fetchAlbum(albumId: string): CachableQuery<AlbumCompleteInfos> {
  return queryApiKeyed(`/album/${albumId}`, null, albumCompleteInfos)
}

export function fetchAlbumTracks(albumId: string): CachableQuery<TrackCompleteInfos[]> {
  return queryApiKeyed(`/album/${albumId}/tracks`, null, trackCompleteInfos.array())
}

export function fetchArtists(
  params: Pagination & { sortBy: ArtistsSort },
): CachableQuery<Paginated<ArtistCompleteInfos>> {
  return queryApiKeyed('/artists', params, paginated(artistCompleteInfos))
}

export function fetchArtistsWithAlbums(
  params: Pagination & { sortBy: ArtistsSort },
): CachableQuery<Paginated<ArtistCompleteInfos>> {
  return queryApiKeyed('/artists/with-albums', params, paginated(artistCompleteInfos))
}

export function fetchArtist(artistId: string): CachableQuery<ArtistCompleteInfos> {
  return queryApiKeyed(`/artist/${artistId}`, null, artistCompleteInfos)
}

export function fetchArtistAlbums(
  artistId: string,
  sortBy: AlbumsSort,
  pagination: Pagination,
): CachableQuery<Paginated<AlbumCompleteInfos>> {
  return queryApiKeyed(
    `/artist/${artistId}/albums`,
    { sortBy, ...pagination },
    paginated(albumCompleteInfos),
  )
}

export function fetchArtistAlbumParticipations(
  artistId: string,
  sortBy: AlbumsSort,
  pagination: Pagination,
): CachableQuery<Paginated<AlbumCompleteInfos>> {
  return queryApiKeyed(
    `/artist/${artistId}/album-participations`,
    { sortBy, ...pagination },
    paginated(albumCompleteInfos),
  )
}

export function fetchArtistTrackParticipations(
  artistId: string,
  sortBy: TracksSort,
  pagination: Pagination,
): CachableQuery<Paginated<TrackCompleteInfos>> {
  return queryApiKeyed(
    `/artist/${artistId}/track-participations`,
    { sortBy, ...pagination },
    paginated(trackCompleteInfos),
  )
}

export function fetchGenres(
  params: Pagination & { sortBy: GenresSort },
): CachableQuery<Paginated<GenreCompleteInfos>> {
  return queryApiKeyed('/genres', params, paginated(genreCompleteInfos))
}

export function fetchGenre(genreId: string): CachableQuery<GenreCompleteInfos> {
  return queryApiKeyed(`/genre/${genreId}`, null, genreCompleteInfos)
}

export function fetchGenreAlbums(
  genreId: string,
  params: Pagination & { sortBy: AlbumsSort },
): CachableQuery<Paginated<AlbumCompleteInfos>> {
  return queryApiKeyed(`/genre/${genreId}/albums`, params, paginated(albumCompleteInfos))
}

export function fetchTracks(
  params: Pagination & { sortBy: TracksSort },
): CachableQuery<Paginated<TrackCompleteInfos>> {
  return queryApiKeyed('/tracks', params, paginated(trackCompleteInfos))
}

export function fetchMultiTracks(trackIds: string[]): CachableQuery<TrackCompleteInfos[]> {
  return queryApiKeyed('/tracks/multi', { ids: trackIds }, trackCompleteInfos.array())
}

export function fetchTrack(trackId: string): CachableQuery<TrackCompleteInfos> {
  return queryApiKeyed(`/tracks/${trackId}`, null, trackCompleteInfos)
}

export function mixTracks(
  params: UserMixParams,
  pagination: Omit<Pagination, 'dir'>,
): Promise<Paginated<TrackCompleteInfos>> {
  return queryApi(
    '/mix',
    { mixParams: JSON.stringify(params), ...pagination, dir: 'ASC' },
    paginated(trackCompleteInfos),
  )
}
