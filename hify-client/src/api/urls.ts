import { API_DOMAIN } from './fetch'
import type { Album, Artist, ArtSize, Genre } from './types'

export const urls = {
  albumArt: (album: Album, size: ArtSize) => `${API_DOMAIN}/album/${album.id}/art?size=${size}`,

  artistArt: (artist: Artist, size: ArtSize) =>
    `${API_DOMAIN}/artist/${artist.id}/art?size=${size}`,

  genreArt: (genre: Genre, size: ArtSize) => `${API_DOMAIN}/genre/${genre.id}/art?size=${size}`,

  trackAudioUrl: (trackId: string) => `${API_DOMAIN}/track/${trackId}/audio`,
}
