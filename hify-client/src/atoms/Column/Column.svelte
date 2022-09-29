<script>
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
</script>

<div class="column">
  <NavigableList>
    <slot />
  </NavigableList>
</div>

<style>
  .column {
    display: flex;
    flex-direction: column;
    overflow-x: auto;
  }
</style>
