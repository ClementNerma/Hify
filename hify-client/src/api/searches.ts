import { queryApi } from './fetch'
import {
  albumCompleteInfos,
  artistCompleteInfos,
  paginated,
  trackCompleteInfos,
  type AlbumCompleteInfos,
  type ArtistCompleteInfos,
  type Paginated,
  type Pagination,
  type TrackCompleteInfos,
} from './types'

export function searchTracks(
  query: string,
  pagination: Pagination,
): Promise<Paginated<TrackCompleteInfos>> {
  return queryApi('/tracks/search', { query, ...pagination }, paginated(trackCompleteInfos))
}

export function searchAlbums(
  query: string,
  pagination: Pagination,
): Promise<Paginated<AlbumCompleteInfos>> {
  return queryApi('/albums/search', { query, ...pagination }, paginated(albumCompleteInfos))
}

export function searchArtists(
  query: string,
  pagination: Pagination,
): Promise<Paginated<ArtistCompleteInfos>> {
  return queryApi('/artists/search', { query, ...pagination }, paginated(artistCompleteInfos))
}
