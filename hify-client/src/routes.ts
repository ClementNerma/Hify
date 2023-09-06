export const ROUTES = {
	home: '/',
	albums: '/albums',
	artists: '/artists',
	genres: '/genres',
	album: (id: string) => `/album/${id}`,
	artist: (id: string) => `/artist/${id}`,
	genre: (id: string) => `/genre/${id}`,
	history: '/history',
	nowPlaying: '/playing',
	search: '/search',
	searchTerms: (terms: string) => `/search/${encodeURIComponent(terms)}`,
	devTools: '/dev',
}
