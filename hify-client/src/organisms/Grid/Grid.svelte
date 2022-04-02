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
    display: grid;
    grid-template-columns: 16.6% 16.6% 16.6% 16.6% 16.6% 16.6%;
    grid-gap: 2% 0;
  }
</style>
