export const ROUTES = {
  home: '/',
  albums: '/albums',
  album: (id: string) => '/albums/' + id,
  artist: (id: string) => '/artists/' + id,
  nowPlaying: '/playing',
  search: '/search',
  searchTerms: (terms: string) => '/search/' + encodeURIComponent(terms),
}
