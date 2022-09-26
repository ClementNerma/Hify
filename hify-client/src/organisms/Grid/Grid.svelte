<script lang="ts">
  import { NavigableGridProps } from '../../navigable/NavigableGrid/NavigableGrid'
  import NavigableGrid from '../../navigable/NavigableGrid/NavigableGrid.svelte'

  export let columns: number
  export let lazyLoader: NavigableGridProps['lazyLoader'] = undefined

  const columnsSize = new Array(columns).fill(`${100 / columns}%`).join(' ')
</script>

<div class="container" style="--columns-size: {columnsSize}">
  <NavigableGrid {columns} {lazyLoader}>
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
