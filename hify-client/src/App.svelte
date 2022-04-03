<script lang="ts">
  import { Router, Route } from 'svelte-navigator'

  import Template from './pages/Template/Template.svelte'
  import NotFound from './pages/NotFoundPage/NotFoundPage.svelte'

  import Home from './pages/HomePage/HomePage.svelte'

  import AlbumsPage from './pages/AlbumsPage/AlbumsPage.svelte'
  import AlbumPage from './pages/AlbumPage/AlbumPage.svelte'

  import NowPlaying from './pages/NowPlayingPage/NowPlayingPage.svelte'
</script>

<Router primary={false}>
  <Template>
    <Route path="/">
      <Home />
    </Route>

    <Route path="/albums">
      <AlbumsPage />
    </Route>

    <Route path="/album/:id" let:params>
      <AlbumPage albumId={params['id']} />
    </Route>

    <Route path="/playing">
      <NowPlaying />
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
