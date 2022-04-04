<script lang="ts">
  import Card from '../../molecules/Card/Card.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'

  export let items: Card['$$prop_def'][]
</script>

<div class="card-row">
  <NavigableRow>
    {#each items as item}
      <Card {...item} />
    {/each}
  </NavigableRow>
</div>

<style>
  .card-row {
    display: flex;
    flex-direction: row;
  }
</style>
