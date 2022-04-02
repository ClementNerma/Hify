<script lang="ts">
  import NavigableGrid from '../../navigable/NavigableGrid/NavigableGrid.svelte'
  import Card from '../../molecules/Card/Card.svelte'

  export let columns: number

  const columnsSize = new Array(columns).fill(`${100 / columns}%`).join(' ')
</script>

<div class="container" style="--columns-size: {columnsSize}">
  <NavigableGrid {columns}>
    <slot />
  </NavigableGrid>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: var(--columns-size);
  }
</style>
