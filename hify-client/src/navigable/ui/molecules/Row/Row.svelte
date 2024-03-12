<script lang="ts">
import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
import { NavigableRowProps } from '../../../headless/NavigableRow/NavigableRow'

export let trapped: NavigableRowProps['trapped'] = undefined
</script>

<div class="row">
  <NavigableRow {trapped}>
    <slot />
  </NavigableRow>
</div>

<style>
  .row {
    overflow-x: hidden;
    display: flex;
    flex-direction: row;
  }
</style>
