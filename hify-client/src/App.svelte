<script lang="ts">
import '@navigable/input-manager'

import { Router, Route } from 'svelte-navigator'
import { ROUTES } from '@root/routes'

import Template from '@pages/Template/Template.svelte'
import NotFound from '@pages/NotFoundPage/NotFoundPage.svelte'

import Home from '@pages/HomePage/HomePage.svelte'

import AlbumsPage from '@pages/AlbumsPage/AlbumsPage.svelte'

import AlbumPage from '@pages/AlbumPage/AlbumPage.svelte'
import ArtistPage from '@pages/ArtistPage/ArtistPage.svelte'
import ArtistsPage from '@pages/ArtistsPage/ArtistsPage.svelte'
import HistoryPage from '@pages/HistoryPage/HistoryPage.svelte'
import NowPlaying from '@pages/NowPlayingPage/NowPlayingPage.svelte'
import SearchPage from '@pages/SearchPage/SearchPage.svelte'
import DevToolsPage from '@pages/DevToolsPage/DevToolsPage.svelte'
import GenresPage from '@pages/GenresPage/GenresPage.svelte'
import GenrePage from '@pages/GenrePage/GenrePage.svelte'
import PlaylistsPage from '@pages/PlaylistsPage/PlaylistsPage.svelte'
import PlaylistPage from '@pages/PlaylistPage/PlaylistPage.svelte'
</script>

<Router primary={false}>
  <Template>
    <Route path={ROUTES.home}>
      <Home />
    </Route>

    <Route path={ROUTES.albums}>
      <AlbumsPage />
    </Route>

    <Route path={ROUTES.artists}>
      <ArtistsPage />
    </Route>

    <Route path={ROUTES.genres}>
      <GenresPage />
    </Route>

    <Route path={ROUTES.playlists}>
      <PlaylistsPage />
    </Route>

    <Route path={ROUTES.album(':id')} let:params>
      <AlbumPage albumId={params['id']} />
    </Route>

    <Route path={ROUTES.artist(':id')} let:params>
      <ArtistPage artistId={params['id']} />
    </Route>

    <Route path={ROUTES.genre(':id')} let:params>
      <GenrePage genreId={params['id']} />
    </Route>

    <Route path={ROUTES.playlist(':id')} let:params>
      <PlaylistPage playlistId={params['id']} />
    </Route>

    <Route path={ROUTES.history}>
      <HistoryPage />
    </Route>

    <Route path={ROUTES.nowPlaying}>
      <NowPlaying />
    </Route>

    <Route path={ROUTES.search}>
      <SearchPage />
    </Route>

    <Route path={ROUTES.search + '/:terms'} let:params>
      <SearchPage searchTerms={decodeURIComponent(params['terms'])} />
    </Route>

    <Route path={ROUTES.devTools}>
      <DevToolsPage />
    </Route>

    <Route>
      <NotFound />
    </Route>
  </Template>
</Router>
