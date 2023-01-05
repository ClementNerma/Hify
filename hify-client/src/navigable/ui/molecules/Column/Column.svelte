<script lang="ts">
  import { onMount } from 'svelte'
  import NavigableList from '../../../headless/NavigableList/NavigableList.svelte'
  import { RequestFocus } from '../../../navigation'

  let requestFocus: RequestFocus

  export let getRequestFocus: ((requestFocus: RequestFocus) => void) | null = null

  onMount(() => getRequestFocus?.(requestFocus))
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
