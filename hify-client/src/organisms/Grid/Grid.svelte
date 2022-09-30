<script lang="ts">
  import { NavigableGrid as Nav } from '../../navigable/NavigableGrid/NavigableGrid'
  import NavigableGrid from '../../navigable/NavigableGrid/NavigableGrid.svelte'
  import { Props } from '../../navigable/navigation'

  export let columns: Props<Nav>['columns']
  export let lazyLoader: Props<Nav>['lazyLoader'] = undefined

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
