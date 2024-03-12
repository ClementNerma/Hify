<script lang="ts">
import type { NavigableGrid as Nav } from '../../../headless/NavigableGrid/NavigableGrid'
import NavigableGrid from '../../../headless/NavigableGrid/NavigableGrid.svelte'
import type { Props } from '../../../navigation'

export let columns: Props<Nav>['columns']
export let lazyLoader: Props<Nav>['lazyLoader'] = undefined
export let distanceBeforeLazyLoading: Props<Nav>['distanceBeforeLazyLoading'] = undefined

const columnsSize = new Array(columns).fill(`${100 / columns}%`).join(' ')
</script>

<div class="container" style="--columns-size: {columnsSize}">
  <NavigableGrid {columns} {lazyLoader} {distanceBeforeLazyLoading}>
    <slot />
  </NavigableGrid>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: var(--columns-size);
    text-align: center;
  }
</style>
