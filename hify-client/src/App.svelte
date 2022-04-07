<script lang="ts">
  import './navigable/input-manager'

  import { Router, Route } from 'svelte-navigator'
  import { ROUTES } from './routes'

  import Template from './pages/Template/Template.svelte'
  import NotFound from './pages/NotFoundPage/NotFoundPage.svelte'

  import Home from './pages/HomePage/HomePage.svelte'

  import AlbumsPage from './pages/AlbumsPage/AlbumsPage.svelte'

  import AlbumPage from './pages/AlbumPage/AlbumPage.svelte'
  import ArtistPage from './pages/ArtistPage/ArtistPage.svelte'

  import NowPlaying from './pages/NowPlayingPage/NowPlayingPage.svelte'

  import SearchPage from './pages/SearchPage/SearchPage.svelte'
  import DevToolsPage from './pages/DevToolsPage/DevToolsPage.svelte'
</script>

<Router primary={false}>
  <Template>
    <Route path={ROUTES.home}>
      <Home />
    </Route>

    <Route path={ROUTES.albums}>
      <AlbumsPage />
    </Route>

    <Route path={ROUTES.album(':id')} let:params>
      <AlbumPage albumId={params['id']} />
    </Route>

    <Route path={ROUTES.artist(':id')} let:params>
      <ArtistPage artistId={params['id']} />
    </Route>

    <Route path={ROUTES.nowPlaying}>
      <NowPlaying />
    </Route>

    <Route path={ROUTES.search}>
      <SearchPage />
    </Route>

    <Route path={ROUTES.searchTerms(':terms')} let:params>
      <SearchPage searchTerms={params['terms']} />
    </Route>

    <Route path={ROUTES.devTools}>
      <DevToolsPage />
    </Route>

    <Route>
      <NotFound />
    </Route>
  </Template>
</Router>
