<script lang="ts">
import { navigate } from 'svelte-navigator'

import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
</script>

<SimpleNavigableItem onPress={() => navigate(-1)}>
  <h1>Page not found, press to go back.</h1>
</SimpleNavigableItem>

<style>
  h1 {
    display: inline-block;
    padding: 10px;
  }
</style>
