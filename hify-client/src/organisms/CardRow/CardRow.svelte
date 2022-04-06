<script lang="ts">
  import Card from '../../molecules/Card/Card.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'

  export let items: Array<Card['$$prop_def'] & { _key: string }>

  const propsWithoutKey = <T extends object>({ _key, ...propsWithoutKey }: T & { _key: string }) => propsWithoutKey
</script>

<div class="card-row">
  <NavigableRow>
    {#each items as item, i (item._key)}
      <Card {...propsWithoutKey(item)} position={i} />
    {/each}
  </NavigableRow>
</div>

<style>
  .card-row {
    display: flex;
    flex-direction: row;
  }
</style>
