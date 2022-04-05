<script lang="ts">
  import NavigableGrid from '../../navigable/NavigableGrid/NavigableGrid.svelte'

  export let columns: number
  export let lazyLoader: NavigableGrid['$$prop_def']['lazyLoader'] = undefined

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
  }
</style>
