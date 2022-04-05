export const ROUTES = {
  home: '/',
  albums: '/albums',
  album: (id: string) => '/albums/' + id,
  nowPlaying: '/playing',
  search: '/search',
  searchTerms: (terms: string) => '/search/' + encodeURIComponent(terms),
}
