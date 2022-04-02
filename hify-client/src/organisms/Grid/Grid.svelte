<script lang="ts">
  import NavigableRow from '../../atoms/NavigableRow.svelte'
  import Card from '../../molecules/Card/Card.svelte'

  export let items: Card['$$prop_def'][]

  const chunks = new Array(Math.ceil(items.length / 6)).fill(null).map((_, i) => items.slice(i * 6, i * 6 + 6))
</script>

<div class="container">
  {#each chunks as chunk}
    <NavigableRow>
      {#each chunk as item}
        <Card {...item} />
      {/each}
    </NavigableRow>
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 16.6% 16.6% 16.6% 16.6% 16.6% 16.6%;
    /*grid-gap: 2% 0;*/
  }
</style>
