<script lang="ts">
  import { Router, Route } from 'svelte-navigator'
  import NavigablePage from './navigable/NavigablePage/NavigablePage.svelte'

  import AlbumsGrid from './pages/AlbumsGrid/AlbumsGrid.svelte'
  import AlbumPage from './pages/AlbumPage/AlbumPage.svelte'
  import Home from './pages/Home/Home.svelte'
  import Template from './pages/Template/Template.svelte'
  import NotFound from './pages/NotFound/NotFound.svelte'
</script>

<NavigablePage>
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
</NavigablePage>

<style>
  :global(body) {
    background-color: lightgray;
    overflow: hidden;
  }
</style>
