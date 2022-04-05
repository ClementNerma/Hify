<script lang="ts">
  import { Router, Route } from 'svelte-navigator'

  import Template from './pages/Template/Template.svelte'
  import NotFound from './pages/NotFoundPage/NotFoundPage.svelte'

  import Home from './pages/HomePage/HomePage.svelte'

  import AlbumsPage from './pages/AlbumsPage/AlbumsPage.svelte'
  import AlbumPage from './pages/AlbumPage/AlbumPage.svelte'

  import NowPlaying from './pages/NowPlayingPage/NowPlayingPage.svelte'
  import SearchPage from './pages/SearchPage/SearchPage.svelte'
  import { ROUTES } from './routes'
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

    <Route path={ROUTES.nowPlaying}>
      <NowPlaying />
    </Route>

    <Route path={ROUTES.search}>
      <SearchPage />
    </Route>

    <Route path={ROUTES.searchTerms(':terms')} let:params>
      <SearchPage searchTerms={params['terms']} />
    </Route>

    <Route>
      <NotFound />
    </Route>
  </Template>
</Router>

<style>
  :global(body) {
    background-color: lightgray;
    overflow: hidden;
  }
</style>
