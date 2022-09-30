<script lang="ts">
  import NavigableList from '../../../headless/NavigableList/NavigableList.svelte'
  import { RequestFocus } from '../../../navigation'

  let _requestFocus: RequestFocus

  export const requestFocus = () => _requestFocus()
</script>

<div class="column">
  <NavigableList bind:requestFocus={_requestFocus}>
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
