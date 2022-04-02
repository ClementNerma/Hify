<script lang="ts">
  import NavigableGrid from '../../navigable/NavigableGrid/NavigableGrid.svelte'
  import Card from '../../molecules/Card/Card.svelte'

  export let items: Card['$$prop_def'][]
</script>

<div class="container">
  <NavigableGrid columns={6}>
    {#each items as item}
      <Card {...item} />
    {/each}
  </NavigableGrid>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 16.6% 16.6% 16.6% 16.6% 16.6% 16.6%;
    /*grid-gap: 2% 0;*/
  }
</style>
