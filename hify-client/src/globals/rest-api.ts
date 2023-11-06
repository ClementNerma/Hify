import { API_SERVER_URL } from '@root/apollo-client'

export const getStreamUri = (trackId: string) => `${API_SERVER_URL}/stream/${trackId}`
export const getArtUri = (artId: string) => `${API_SERVER_URL}/art/${artId}`
export const getArtistArtUri = (artistId: string) => `${API_SERVER_URL}/art/artist/${artistId}`
