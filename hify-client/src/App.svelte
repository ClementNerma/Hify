<script lang="ts">
  import { Router, Route } from 'svelte-navigator'
  import Home from './organisms/Home/Home.svelte'
</script>

<Router>
  <Route path="/">
    <Home />
  </Route>
</Router>
