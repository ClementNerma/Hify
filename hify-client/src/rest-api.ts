import { API_SERVER_URL } from './apollo-client'

export const getStreamUri = (trackId: string) => `${API_SERVER_URL}/stream/${trackId}`
export const getAlbumArtUri = (albumId: string) => `${API_SERVER_URL}/art/${albumId}`
