import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomePage.vue'

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: '/',
			name: 'home',
			component: HomeView,
		},
		{
			path: '/now-playing',
			name: 'now-playing',
			component: () => import('../views/NowPlayingPage.vue'),
		},
		{
			path: '/search/:query?',
			name: 'search',
			component: () => import('../views/SearchPage.vue'),
		},
		{
			path: '/history',
			name: 'history',
			component: () => import('../views/HistoryPage.vue'),
		},
		{
			path: '/devtools',
			name: 'devtools',
			component: () => import('../views/DevToolsPage.vue'),
		},
		{
			path: '/albums',
			name: 'albums',
			component: () => import('../views/AlbumsPage.vue'),
		},
		{
			path: '/album-artists',
			name: 'album-artists',
			component: () => import('../views/AlbumArtistsPage.vue'),
		},
		{
			path: '/artists',
			name: 'artists',
			component: () => import('../views/ArtistsPage.vue'),
		},
		{
			path: '/genres',
			name: 'genres',
			component: () => import('../views/GenresPage.vue'),
		},
		{
			path: '/album/:id',
			name: 'album',
			component: () => import('../views/AlbumPage.vue'),
		},
		{
			path: '/artist/:id',
			name: 'artist',
			component: () => import('../views/ArtistPage.vue'),
		},
		{
			path: '/artist/:id/tracks',
			name: 'artist-tracks',
			component: () => import('../views/ArtistTracksPage.vue'),
		},
		{
			path: '/genre/:id',
			name: 'genre',
			component: () => import('../views/GenrePage.vue'),
		},
	],
})

export default router
