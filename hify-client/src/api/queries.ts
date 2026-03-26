import { queryApi } from './fetch'
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
): Promise<Paginated<AlbumCompleteInfos>> {
  return queryApi('/albums', params, paginated(albumCompleteInfos))
}

export function fetchAlbum(albumId: string): Promise<AlbumCompleteInfos> {
  return queryApi(`/album/${albumId}`, null, albumCompleteInfos)
}

export function fetchAlbumTracks(albumId: string): Promise<TrackCompleteInfos[]> {
  return queryApi(`/album/${albumId}/tracks`, null, trackCompleteInfos.array())
}

export function fetchArtists(
  params: Pagination & { sortBy: ArtistsSort },
): Promise<Paginated<ArtistCompleteInfos>> {
  return queryApi('/artists', params, paginated(artistCompleteInfos))
}

export function fetchArtistsWithAlbums(
  params: Pagination & { sortBy: ArtistsSort },
): Promise<Paginated<ArtistCompleteInfos>> {
  return queryApi('/artists/with-albums', params, paginated(artistCompleteInfos))
}

export function fetchArtist(artistId: string): Promise<ArtistCompleteInfos> {
  return queryApi(`/artist/${artistId}`, null, artistCompleteInfos)
}

export function fetchArtistAlbums(
  artistId: string,
  sortBy: AlbumsSort,
  pagination: Pagination,
): Promise<Paginated<AlbumCompleteInfos>> {
  return queryApi(
    `/artist/${artistId}/albums`,
    { sortBy, ...pagination },
    paginated(albumCompleteInfos),
  )
}

export function fetchArtistAlbumParticipations(
  artistId: string,
  sortBy: AlbumsSort,
  pagination: Pagination,
): Promise<Paginated<AlbumCompleteInfos>> {
  return queryApi(
    `/artist/${artistId}/album-participations`,
    { sortBy, ...pagination },
    paginated(albumCompleteInfos),
  )
}

export function fetchArtistTrackParticipations(
  artistId: string,
  sortBy: TracksSort,
  pagination: Pagination,
): Promise<Paginated<TrackCompleteInfos>> {
  return queryApi(
    `/artist/${artistId}/track-participations`,
    { sortBy, ...pagination },
    paginated(trackCompleteInfos),
  )
}

export function fetchGenres(
  params: Pagination & { sortBy: GenresSort },
): Promise<Paginated<GenreCompleteInfos>> {
  return queryApi('/genres', params, paginated(genreCompleteInfos))
}

export function fetchGenre(genreId: string): Promise<GenreCompleteInfos> {
  return queryApi(`/genre/${genreId}`, null, genreCompleteInfos)
}

export function fetchGenreAlbums(
  genreId: string,
  params: Pagination & { sortBy: AlbumsSort },
): Promise<Paginated<AlbumCompleteInfos>> {
  return queryApi(`/genre/${genreId}/albums`, params, paginated(albumCompleteInfos))
}

export function fetchTracks(
  params: Pagination & { sortBy: TracksSort },
): Promise<Paginated<TrackCompleteInfos>> {
  return queryApi('/tracks', params, paginated(trackCompleteInfos))
}

export function fetchMultiTracks(trackIds: string[]): Promise<TrackCompleteInfos[]> {
  return queryApi('/tracks/multi', { ids: trackIds }, trackCompleteInfos.array())
}

export function fetchTrack(trackId: string): Promise<TrackCompleteInfos> {
  return queryApi(`/tracks/${trackId}`, null, trackCompleteInfos)
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
