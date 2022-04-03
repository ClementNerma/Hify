<script lang="ts">
  import { Router, Route } from 'svelte-navigator'

  import Template from './pages/Template/Template.svelte'
  import NotFound from './pages/NotFound/NotFound.svelte'

  import Home from './pages/Home/Home.svelte'

  import AlbumsGrid from './pages/AlbumsGrid/AlbumsGrid.svelte'
  import AlbumPage from './pages/AlbumPage/AlbumPage.svelte'
</script>

<Router primary={false}>
  <Template>
    <Route path="/">
      <Home />
    </Route>

    <Route path="/albums">
      <AlbumsGrid />
    </Route>

    <Route path="/album/:id" let:params>
      <AlbumPage albumId={params['id']} />
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
