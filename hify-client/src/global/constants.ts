import type { AlbumFragment, ArtistFragment } from '@/graphql/generated/graphql'

export const API_SERVER_URL = `http://${location.hostname}:8893`

export const LARGE_MIX_TRACKS_QTY = 100
export const DISPLAYABLE_ITEMS_QTY = 25
export const MIN_GREAT_RATING = 8
export const EXTEND_MIX_TRACKS_QTY = 20

export const GRID_TRACKS_PER_ROW = 7
export const GRID_TRACKS_PRELOAD_ROWS = 5

export const GRID_ALBUMS_PER_ROW = 6
export const GRID_ALBUMS_PRELOAD_ROWS = 5

export const GRID_ARTISTS_PER_ROW = 6
export const GRID_ARTISTS_PRELOAD_ROWS = 5

export const GRID_FETCH_AHEAD_ROWS = 3
export const LIST_FETCH_AHEAD_ROWS = 5

export const IMG_FALLBACK_URL = 'about:blank'

export function getStreamUrl(trackId: string) {
	return `${API_SERVER_URL}/stream/${trackId}`
}

export function getAlbumArtUrl(album: AlbumFragment) {
	return album.hasArt ? `${API_SERVER_URL}/art/album/${album.id}` : IMG_FALLBACK_URL
}

export function getArtistArtUrl(artist: ArtistFragment) {
	return artist.hasArt ? `${API_SERVER_URL}/art/artist/${artist.id}` : IMG_FALLBACK_URL
}
