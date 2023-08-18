<script lang="ts">
  import { onMount } from 'svelte'
  import NavigableList from '../../../headless/NavigableList/NavigableList.svelte'
  import {NavigableList as NavigableListClass} from '../../../headless/NavigableList/NavigableList'
  import { Props, RequestFocus } from '../../../navigation'

  let requestFocus: RequestFocus

  export let hasFocusPriority: Props<NavigableListClass>['hasFocusPriority'] = null
  export let trapped: Props<NavigableListClass>['trapped'] = undefined

  export let getRequestFocus: ((requestFocus: RequestFocus) => void) | null = null

  onMount(() => getRequestFocus?.(requestFocus))
</script>

<div class="column">
  <NavigableList bind:requestFocus {hasFocusPriority} {trapped}>
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
