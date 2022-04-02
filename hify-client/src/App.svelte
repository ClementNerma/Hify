<script lang="ts">
  import { Router, Route } from 'svelte-navigator'
  import NavigablePage from './navigable/NavigablePage/NavigablePage.svelte'

  import AlbumGrid from './pages/AlbumGrid/AlbumGrid.svelte'
  import AlbumTracks from './pages/AlbumTracks/AlbumTracks.svelte'
  import Home from './pages/Home/Home.svelte'
  import Template from './pages/Template/Template.svelte'
</script>

<NavigablePage>
  <Router>
    <Template>
      <Route path="/">
        <Home />
      </Route>

      <Route path="/albums">
        <AlbumGrid />
      </Route>

      <Route path="/album/:id" let:params>
        <AlbumTracks albumId={params['id']} />
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
