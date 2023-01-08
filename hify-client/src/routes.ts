export const ROUTES = {
	home: '/',
	albums: '/albums',
	artists: '/artists',
	genres: '/genres',
	album: (id: string) => `/albums/${id}`,
	artist: (id: string) => `/artists/${id}`,
	genre: (id: string) => `/genres/${id}`,
	history: '/history',
	nowPlaying: '/playing',
	search: '/search',
	searchTerms: (terms: string) => `/search/${encodeURIComponent(terms)}`,
	devTools: '/dev',
}
