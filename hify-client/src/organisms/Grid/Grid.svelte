<script lang="ts">
  import Card from '../../molecules/Card/Card.svelte'

  export let items: Card['$$prop_def'][]
</script>

<div class="container">
  {#each items as item}
    <Card {...item} />
  {/each}
</div>

<style>
  .container {
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    justify-content: space-between;
    align-items: flex-start;
    align-content: center;
  }
</style>
