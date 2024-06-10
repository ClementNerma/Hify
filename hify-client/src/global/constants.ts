export const API_SERVER_URL = `http://${location.hostname}:8893`

export const LARGE_MIX_TRACKS_QTY = 100
export const DISPLAYABLE_ITEMS_QTY = 25
export const MIN_GREAT_RATING = 8
export const EXTEND_MIX_TRACKS_QTY = 20
export const GRID_TRACKS_PER_ROW = 7

export const getStreamUri = (trackId: string) => `${API_SERVER_URL}/stream/${trackId}`
export const getArtUri = (artId: string) => `${API_SERVER_URL}/art/${artId}`
export const getArtistArtUri = (artistId: string) => `${API_SERVER_URL}/art/artist/${artistId}`
