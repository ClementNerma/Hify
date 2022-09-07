export const ROUTES = {
  home: '/',
  albums: '/albums',
  artists: '/artists',
  album: (id: string) => '/albums/' + id,
  artist: (id: string) => '/artists/' + id,
  history: '/history',
  nowPlaying: '/playing',
  search: '/search',
  searchTerms: (terms: string) => '/search/' + encodeURIComponent(terms),
  devTools: '/dev',
}
