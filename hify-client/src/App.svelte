<script lang="ts">
  import { Router, Route } from 'svelte-navigator'
  import AlbumGrid from './pages/AlbumGrid/AlbumGrid.svelte'
  import AlbumTracks from './pages/AlbumTracks/AlbumTracks.svelte'
  import Home from './pages/Home/Home.svelte'
</script>

<div class="container">
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
</div>

<style>
  .container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: lightgray;
    overflow: auto;
  }
</style>
