import { buildRoutes, route } from './router/routes'

export const routes = buildRoutes({
  home: route('/'),

  albums: route('/albums'),
  album: route('/album/:albumId'),

  artists: route('/artists'),
  artist: route('/artist/:artistId'),

  genres: route('/genres'),
  genre: route('/genre/:genreId'),

  search: route('/search/:initialQuery?'),

  player: route('/player'),
  history: route('/history'),
  tools: route('/tools'),
})

export type Routes = typeof routes
