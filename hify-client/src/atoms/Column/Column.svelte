<script lang="ts">
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import { RequestFocus } from '../../navigable/navigation'

  export let requestFocus: RequestFocus
</script>

<div class="column">
  <NavigableList bind:requestFocus>
    <slot />
  </NavigableList>
</div>

<style>
  .column {
    display: flex;
    flex-direction: column;
    overflow-x: auto;
  }
</style>
