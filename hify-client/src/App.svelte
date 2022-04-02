<script lang="ts">
  import { Router, Route } from 'svelte-navigator'
  import AlbumGrid from './pages/AlbumGrid/AlbumGrid.svelte'
  import AlbumTracks from './pages/AlbumTracks/AlbumTracks.svelte'
  import Home from './pages/Home/Home.svelte'
</script>

<Router>
  <Route path="/">
    <Home />
  </Route>

  <Route path="/albums">
    <AlbumGrid />
  </Route>

  <Route path="/album/:id" let:params>
    <AlbumTracks albumId={params['id']} />
  </Route>
</Router>

<style>
  :global(body) {
    background-color: lightgray;
    overflow: hidden;
  }
</style>
